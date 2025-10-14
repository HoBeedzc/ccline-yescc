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
struct DailyUsageApiResponse {
    daily_usage: Vec<DailyUsage>,
}

#[derive(Debug, Deserialize)]
struct DailyUsage {
    #[allow(dead_code)]
    date: String,
    total_cost: f64,
}

#[derive(Debug, Deserialize)]
struct BalanceApiResponse {
    #[allow(dead_code)]
    balance: f64,
    #[allow(dead_code)]
    pay_as_you_go_balance: f64,
    #[allow(dead_code)]
    subscription_balance: f64,
    total_balance: f64,
    weekly_limit: f64,
    weekly_spent_balance: f64,
}

// 端点配置
#[derive(Debug, Clone)]
struct EndpointConfig {
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
struct SmartEndpointDetector;

impl SmartEndpointDetector {
    fn get_daily_usage_endpoint() -> EndpointConfig {
        EndpointConfig {
            url: "https://co.yes.vg/api/v1/user/usage/daily".to_string(),
            name: "daily_usage".to_string(),
        }
    }

    fn get_balance_endpoint() -> EndpointConfig {
        EndpointConfig {
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

    fn fetch_daily_usage(api_key: &str) -> Option<DailyUsageApiResponse> {
        let endpoint = Self::get_daily_usage_endpoint();
        let debug = env::var("YESCODE_DEBUG").is_ok();

        if debug {
            eprintln!("[DEBUG] Fetching daily usage from: {}", endpoint.url);
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

                    response.into_json::<DailyUsageApiResponse>().ok()
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

    fn fetch_balance(api_key: &str) -> Option<BalanceApiResponse> {
        let endpoint = Self::get_balance_endpoint();
        let debug = env::var("YESCODE_DEBUG").is_ok();

        if debug {
            eprintln!("[DEBUG] Fetching balance from: {}", endpoint.url);
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
                            "[DEBUG] Success: {} in {}ms",
                            endpoint.name,
                            elapsed.as_millis()
                        );
                    }

                    response.into_json::<BalanceApiResponse>().ok()
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

    fn format_daily_used_total(&self, daily_used: f64, total: f64) -> String {
        format!("${:.2}/${:.2}", daily_used, total)
    }

    fn format_week_limit(&self, weekly_used: f64, limit: f64) -> String {
        format!("Week: ${:.2}/${:.0}", weekly_used, limit)
    }

    fn get_today_cost(&self, response: &DailyUsageApiResponse) -> f64 {
        response
            .daily_usage
            .first()
            .map(|usage| usage.total_cost)
            .unwrap_or(0.0)
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

            // 获取今日使用量
            let daily_usage_response = SmartEndpointDetector::fetch_daily_usage(&api_key);
            let today_cost = daily_usage_response
                .as_ref()
                .map(|r| self.get_today_cost(r))
                .unwrap_or(0.0);

            // 获取余额信息
            if let Some(balance_response) = SmartEndpointDetector::fetch_balance(&api_key) {
                // 第一块：今日已用 / 总余额
                let primary = self.format_daily_used_total(today_cost, balance_response.total_balance);

                // 第二块：本周已用 / 周限制
                let secondary = self.format_week_limit(
                    balance_response.weekly_spent_balance,
                    balance_response.weekly_limit,
                );

                let mut metadata = HashMap::new();
                metadata.insert("daily_spent".to_string(), today_cost.to_string());
                metadata.insert(
                    "total_balance".to_string(),
                    balance_response.total_balance.to_string(),
                );
                metadata.insert(
                    "weekly_spent".to_string(),
                    balance_response.weekly_spent_balance.to_string(),
                );
                metadata.insert(
                    "weekly_limit".to_string(),
                    balance_response.weekly_limit.to_string(),
                );

                Some(SegmentData {
                    primary,
                    secondary,
                    metadata,
                })
            } else {
                // API调用失败
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
