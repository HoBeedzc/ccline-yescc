use super::{Segment, SegmentData};
use crate::config::{InputData, SegmentId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

// API 响应结构
#[derive(Debug, Deserialize)]
struct YesCodeApiResponse {
    daily_usage: Vec<DailyUsage>,
}

#[derive(Debug, Deserialize)]
struct BalanceApiResponse {
    balance: f64,
    pay_as_you_go_balance: f64,
    subscription_balance: f64,
    total_balance: f64,
}

#[derive(Debug, Deserialize)]
struct DailyUsage {
    #[allow(dead_code)]
    date: String,
    total_cost: f64,
}

// 端点配置
#[derive(Debug, Clone)]
struct EndpointConfig {
    url: String,
    name: String,
}

#[derive(Debug, Clone)]
struct BalanceEndpointConfig {
    url: String,
    name: String,
}

// 端点缓存
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct EndpointCache {
    api_key_hash: u64,
    successful_endpoint: String,
    last_success_time: SystemTime,
    success_count: u32,
}

// 智能端点检测器
struct SmartEndpointDetector {
    endpoints: Vec<EndpointConfig>,
}

impl SmartEndpointDetector {
    fn new() -> Self {
        let endpoints = vec![EndpointConfig {
            url: "https://co.yes.vg/api/v1/user/usage/daily".to_string(),
            name: "main".to_string(),
        }];

        Self { endpoints }
    }

    fn get_balance_endpoint() -> BalanceEndpointConfig {
        BalanceEndpointConfig {
            url: "https://co.yes.vg/api/v1/user/balance".to_string(),
            name: "balance".to_string(),
        }
    }

    #[allow(dead_code)]
    fn get_cache_file_path() -> PathBuf {
        if let Some(home) = dirs::home_dir() {
            home.join(".claude")
                .join("ccline")
                .join("endpoint_cache.json")
        } else {
            PathBuf::from("endpoint_cache.json")
        }
    }

    #[allow(dead_code)]
    fn hash_api_key(api_key: &str) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        api_key.hash(&mut hasher);
        hasher.finish()
    }

    fn try_endpoint(&self, endpoint: &EndpointConfig, api_key: &str) -> Option<YesCodeApiResponse> {
        let debug = env::var("YESCODE_DEBUG").is_ok();

        if debug {
            eprintln!("[DEBUG] Trying endpoint: {}", endpoint.url);
        }

        let start_time = SystemTime::now();
        let result = ureq::get(&endpoint.url)
            .set("accept", "*/*")
            .set("content-type", "application/json")
            .set("X-API-Key", api_key)
            .timeout(Duration::from_secs(5))
            .call();

        match result {
            Ok(response) => {
                if response.status() == 200 {
                    let elapsed = start_time.elapsed().unwrap_or(Duration::from_secs(0));
                    if debug {
                        eprintln!(
                            "[DEBUG] Success: {} in {}ms",
                            endpoint.name,
                            elapsed.as_millis()
                        );
                    }

                    response.into_json::<YesCodeApiResponse>().ok()
                } else {
                    if debug {
                        eprintln!(
                            "[DEBUG] Failed: {} status {}",
                            endpoint.name,
                            response.status()
                        );
                    }
                    None
                }
            }
            Err(e) => {
                if debug {
                    eprintln!("[DEBUG] Error: {} - {}", endpoint.name, e);
                }
                None
            }
        }
    }

    fn detect_endpoint(&mut self, api_key: &str) -> Option<(String, YesCodeApiResponse)> {
        // 尝试所有端点
        let endpoints_clone = self.endpoints.clone();
        for endpoint in &endpoints_clone {
            if let Some(response) = self.try_endpoint(endpoint, api_key) {
                return Some((endpoint.url.clone(), response));
            }
        }

        None
    }

    fn detect_endpoint_static(api_key: &str) -> Option<(String, YesCodeApiResponse)> {
        let mut detector = SmartEndpointDetector::new();
        detector.detect_endpoint(api_key)
    }

    fn try_balance_endpoint(api_key: &str) -> Option<BalanceApiResponse> {
        let endpoint = Self::get_balance_endpoint();
        let debug = env::var("YESCODE_DEBUG").is_ok();

        if debug {
            eprintln!("[DEBUG] Trying balance endpoint: {}", endpoint.url);
        }

        let start_time = SystemTime::now();
        let result = ureq::get(&endpoint.url)
            .set("accept", "application/json")
            .set("X-API-Key", api_key)
            .timeout(Duration::from_secs(5))
            .call();

        match result {
            Ok(response) => {
                if response.status() == 200 {
                    let elapsed = start_time.elapsed().unwrap_or(Duration::from_secs(0));
                    if debug {
                        eprintln!(
                            "[DEBUG] Balance Success: {} in {}ms",
                            endpoint.name,
                            elapsed.as_millis()
                        );
                    }

                    response.into_json::<BalanceApiResponse>().ok()
                } else {
                    if debug {
                        eprintln!(
                            "[DEBUG] Balance Failed: {} status {}",
                            endpoint.name,
                            response.status()
                        );
                    }
                    None
                }
            }
            Err(e) => {
                if debug {
                    eprintln!("[DEBUG] Balance Error: {} - {}", endpoint.name, e);
                }
                None
            }
        }
    }
}

#[derive(Default)]
pub struct QuotaSegment;

impl QuotaSegment {
    pub fn new() -> Self {
        Self
    }

    fn load_api_key(&self) -> Option<String> {
        // 优先级：环境变量 > Claude Code settings.json > api_key 文件

        // 1. 环境变量
        if let Ok(key) = env::var("YESCODE_API_KEY") {
            return Some(key);
        }

        if let Ok(key) = env::var("ANTHROPIC_API_KEY") {
            return Some(key);
        }

        if let Ok(key) = env::var("ANTHROPIC_AUTH_TOKEN") {
            return Some(key);
        }

        // 2. Claude Code settings.json
        if let Some(key) = self.load_from_settings() {
            return Some(key);
        }

        // 3. api_key 文件
        if let Some(home) = dirs::home_dir() {
            let api_key_path = home.join(".claude").join("api_key");
            if let Ok(key) = fs::read_to_string(api_key_path) {
                return Some(key.trim().to_string());
            }
        }

        None
    }

    fn load_from_settings(&self) -> Option<String> {
        if let Some(home) = dirs::home_dir() {
            let settings_path = home.join(".claude").join("settings.json");
            if let Ok(content) = fs::read_to_string(settings_path) {
                if let Ok(settings) = serde_json::from_str::<serde_json::Value>(&content) {
                    if let Some(env) = settings.get("env") {
                        if let Some(token) = env.get("ANTHROPIC_AUTH_TOKEN") {
                            if let Some(token_str) = token.as_str() {
                                return Some(token_str.to_string());
                            }
                        }
                        if let Some(key) = env.get("ANTHROPIC_API_KEY") {
                            if let Some(key_str) = key.as_str() {
                                return Some(key_str.to_string());
                            }
                        }
                    }
                }
            }
        }
        None
    }

    fn format_daily_spent(&self, spent: f64) -> String {
        format!("Used:${:.2}", spent)
    }

    fn format_balance(&self, balance: f64) -> String {
        format!("Left:${:.2}", balance)
    }

    fn get_today_cost(&self, response: &YesCodeApiResponse) -> Option<f64> {
        // 获取最新一天的数据（假设数组是按日期排序的）
        response.daily_usage.first().map(|usage| usage.total_cost)
    }
}

impl Segment for QuotaSegment {
    fn collect(&self, _input: &InputData) -> Option<SegmentData> {
        #[cfg(not(feature = "quota"))]
        {
            return None;
        }

        #[cfg(feature = "quota")]
        {
            let api_key = self.load_api_key()?;

            // 使用静态方法进行端点检测
            if let Some((endpoint_url, response)) =
                SmartEndpointDetector::detect_endpoint_static(&api_key)
            {
                if let Some(today_cost) = self.get_today_cost(&response) {
                    let daily_spent = self.format_daily_spent(today_cost);

                    // 尝试获取余额信息
                    let secondary = if let Some(balance_response) =
                        SmartEndpointDetector::try_balance_endpoint(&api_key) {
                        self.format_balance(balance_response.total_balance)
                    } else {
                        "Balance Unknown".to_string()
                    };

                    let mut metadata = HashMap::new();
                    metadata.insert("raw_spent".to_string(), today_cost.to_string());
                    metadata.insert("endpoint_used".to_string(), endpoint_url);

                    Some(SegmentData {
                        primary: daily_spent,
                        secondary,
                        metadata,
                    })
                } else {
                    // 今天没有数据，但仍尝试获取余额
                    let secondary = if let Some(balance_response) =
                        SmartEndpointDetector::try_balance_endpoint(&api_key) {
                        self.format_balance(balance_response.total_balance)
                    } else {
                        "Balance Unknown".to_string()
                    };

                    let mut metadata = HashMap::new();
                    metadata.insert("status".to_string(), "no_data_today".to_string());

                    Some(SegmentData {
                        primary: "Used:$0.00".to_string(),
                        secondary,
                        metadata,
                    })
                }
            } else {
                // 所有端点都失败
                let mut metadata = HashMap::new();
                metadata.insert("status".to_string(), "offline".to_string());

                Some(SegmentData {
                    primary: "Offline".to_string(),
                    secondary: "Offline".to_string(),
                    metadata,
                })
            }
        }
    }

    fn id(&self) -> SegmentId {
        SegmentId::Quota
    }
}
