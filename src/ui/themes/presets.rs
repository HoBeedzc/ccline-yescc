// Theme presets for TUI configuration

use crate::config::{Config, StyleConfig, StyleMode};

// Import all theme modules
use super::{
    theme_cometix, theme_default, theme_gruvbox, theme_minimal, theme_nord, theme_powerline_dark,
    theme_powerline_light, theme_powerline_rose_pine, theme_powerline_tokyo_night,
};

pub struct ThemePresets;

impl ThemePresets {
    pub fn get_theme(theme_name: &str) -> Config {
        // First try to load from file
        if let Ok(config) = Self::load_theme_from_file(theme_name) {
            return config;
        }

        // Fallback to built-in themes
        match theme_name {
            "cometix" => Self::get_cometix(),
            "default" => Self::get_default(),
            "gruvbox" => Self::get_gruvbox(),
            "minimal" => Self::get_minimal(),
            "nord" => Self::get_nord(),
            "powerline-dark" => Self::get_powerline_dark(),
            "powerline-light" => Self::get_powerline_light(),
            "powerline-rose-pine" => Self::get_powerline_rose_pine(),
            "powerline-tokyo-night" => Self::get_powerline_tokyo_night(),
            _ => Self::get_default(),
        }
    }

    /// Load theme from file system
    pub fn load_theme_from_file(theme_name: &str) -> Result<Config, Box<dyn std::error::Error>> {
        let themes_dir = Self::get_themes_path();
        let theme_path = themes_dir.join(format!("{}.toml", theme_name));

        if !theme_path.exists() {
            return Err(format!("Theme file not found: {}", theme_path.display()).into());
        }

        let content = std::fs::read_to_string(&theme_path)?;
        let mut config: Config = toml::from_str(&content)?;

        // Ensure the theme field matches the requested theme
        config.theme = theme_name.to_string();

        Ok(config)
    }

    /// Get the themes directory path (~/.claude/ccline/themes/)
    fn get_themes_path() -> std::path::PathBuf {
        if let Some(home) = dirs::home_dir() {
            home.join(".claude").join("ccline").join("themes")
        } else {
            std::path::PathBuf::from(".claude/ccline/themes")
        }
    }

    /// Save current config as a new theme
    pub fn save_theme(theme_name: &str, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
        let themes_dir = Self::get_themes_path();
        let theme_path = themes_dir.join(format!("{}.toml", theme_name));

        // Create themes directory if it doesn't exist
        std::fs::create_dir_all(&themes_dir)?;

        // Create a copy of config with the correct theme name
        let mut theme_config = config.clone();
        theme_config.theme = theme_name.to_string();

        let content = toml::to_string_pretty(&theme_config)?;
        std::fs::write(&theme_path, content)?;

        Ok(())
    }

    /// List all available themes (built-in + custom)
    pub fn list_available_themes() -> Vec<String> {
        let mut themes = vec![
            "cometix".to_string(),
            "default".to_string(),
            "minimal".to_string(),
            "gruvbox".to_string(),
            "nord".to_string(),
            "powerline-dark".to_string(),
            "powerline-light".to_string(),
            "powerline-rose-pine".to_string(),
            "powerline-tokyo-night".to_string(),
        ];

        // Add custom themes from file system
        if let Ok(themes_dir) = std::fs::read_dir(Self::get_themes_path()) {
            for entry in themes_dir.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".toml") {
                        let theme_name = name.trim_end_matches(".toml").to_string();
                        if !themes.contains(&theme_name) {
                            themes.push(theme_name);
                        }
                    }
                }
            }
        }

        themes
    }

    pub fn get_available_themes() -> Vec<(&'static str, &'static str)> {
        vec![
            ("cometix", "Cometix theme"),
            ("default", "Default theme with emoji icons"),
            ("minimal", "Minimal theme with reduced colors"),
            ("gruvbox", "Gruvbox color scheme"),
            ("nord", "Nord color scheme"),
            ("powerline-dark", "Dark powerline theme"),
            ("powerline-light", "Light powerline theme"),
            ("powerline-rose-pine", "Rose Pine powerline theme"),
            ("powerline-tokyo-night", "Tokyo Night powerline theme"),
        ]
    }

    pub fn get_cometix() -> Config {
        Config {
            style: StyleConfig {
                mode: StyleMode::NerdFont,
                separator: " | ".to_string(),
            },
            segments: vec![
                theme_cometix::model_segment(),
                theme_cometix::directory_segment(),
                theme_cometix::git_segment(),
                theme_cometix::context_window_segment(),
                theme_cometix::usage_segment(),
                theme_cometix::cost_segment(),
                theme_cometix::session_segment(),
                theme_cometix::output_style_segment(),
                theme_cometix::quota_segment(),
            ],
            theme: "cometix".to_string(),
        }
    }

    pub fn get_default() -> Config {
        Config {
            style: StyleConfig {
                mode: StyleMode::Plain,
                separator: " | ".to_string(),
            },
            segments: vec![
                theme_default::model_segment(),
                theme_default::directory_segment(),
                theme_default::git_segment(),
                theme_default::context_window_segment(),
                theme_default::usage_segment(),
                theme_default::cost_segment(),
                theme_default::session_segment(),
                theme_default::output_style_segment(),
                theme_default::quota_segment(),
            ],
            theme: "default".to_string(),
        }
    }

    pub fn get_minimal() -> Config {
        Config {
            style: StyleConfig {
                mode: StyleMode::Plain,
                separator: " │ ".to_string(),
            },
            segments: vec![
                theme_minimal::model_segment(),
                theme_minimal::directory_segment(),
                theme_minimal::git_segment(),
                theme_minimal::context_window_segment(),
                theme_minimal::usage_segment(),
                theme_minimal::cost_segment(),
                theme_minimal::session_segment(),
                theme_minimal::output_style_segment(),
                theme_minimal::quota_segment(),
            ],
            theme: "minimal".to_string(),
        }
    }

    pub fn get_gruvbox() -> Config {
        Config {
            style: StyleConfig {
                mode: StyleMode::NerdFont,
                separator: " | ".to_string(),
            },
            segments: vec![
                theme_gruvbox::model_segment(),
                theme_gruvbox::directory_segment(),
                theme_gruvbox::git_segment(),
                theme_gruvbox::context_window_segment(),
                theme_gruvbox::usage_segment(),
                theme_gruvbox::cost_segment(),
                theme_gruvbox::session_segment(),
                theme_gruvbox::output_style_segment(),
                theme_gruvbox::quota_segment(),
            ],
            theme: "gruvbox".to_string(),
        }
    }

    pub fn get_nord() -> Config {
        Config {
            style: StyleConfig {
                mode: StyleMode::NerdFont,
                separator: "".to_string(),
            },
            segments: vec![
                theme_nord::model_segment(),
                theme_nord::directory_segment(),
                theme_nord::git_segment(),
                theme_nord::context_window_segment(),
                theme_nord::usage_segment(),
                theme_nord::cost_segment(),
                theme_nord::session_segment(),
                theme_nord::output_style_segment(),
                theme_nord::quota_segment(),
            ],
            theme: "nord".to_string(),
        }
    }

    pub fn get_powerline_dark() -> Config {
        Config {
            style: StyleConfig {
                mode: StyleMode::NerdFont,
                separator: "".to_string(),
            },
            segments: vec![
                theme_powerline_dark::model_segment(),
                theme_powerline_dark::directory_segment(),
                theme_powerline_dark::git_segment(),
                theme_powerline_dark::context_window_segment(),
                theme_powerline_dark::usage_segment(),
                theme_powerline_dark::cost_segment(),
                theme_powerline_dark::session_segment(),
                theme_powerline_dark::output_style_segment(),
                theme_powerline_dark::quota_segment(),
            ],
            theme: "powerline-dark".to_string(),
        }
    }

    pub fn get_powerline_light() -> Config {
        Config {
            style: StyleConfig {
                mode: StyleMode::NerdFont,
                separator: "".to_string(),
            },
            segments: vec![
                theme_powerline_light::model_segment(),
                theme_powerline_light::directory_segment(),
                theme_powerline_light::git_segment(),
                theme_powerline_light::context_window_segment(),
                theme_powerline_light::usage_segment(),
                theme_powerline_light::cost_segment(),
                theme_powerline_light::session_segment(),
                theme_powerline_light::output_style_segment(),
                theme_powerline_light::quota_segment(),
            ],
            theme: "powerline-light".to_string(),
        }
    }

    pub fn get_powerline_rose_pine() -> Config {
        Config {
            style: StyleConfig {
                mode: StyleMode::NerdFont,
                separator: "".to_string(),
            },
            segments: vec![
                theme_powerline_rose_pine::model_segment(),
                theme_powerline_rose_pine::directory_segment(),
                theme_powerline_rose_pine::git_segment(),
                theme_powerline_rose_pine::context_window_segment(),
                theme_powerline_rose_pine::usage_segment(),
                theme_powerline_rose_pine::cost_segment(),
                theme_powerline_rose_pine::session_segment(),
                theme_powerline_rose_pine::output_style_segment(),
                theme_powerline_rose_pine::quota_segment(),
            ],
            theme: "powerline-rose-pine".to_string(),
        }
    }

    pub fn get_powerline_tokyo_night() -> Config {
        Config {
            style: StyleConfig {
                mode: StyleMode::NerdFont,
                separator: "".to_string(),
            },
            segments: vec![
                theme_powerline_tokyo_night::model_segment(),
                theme_powerline_tokyo_night::directory_segment(),
                theme_powerline_tokyo_night::git_segment(),
                theme_powerline_tokyo_night::context_window_segment(),
                theme_powerline_tokyo_night::usage_segment(),
                theme_powerline_tokyo_night::cost_segment(),
                theme_powerline_tokyo_night::session_segment(),
                theme_powerline_tokyo_night::output_style_segment(),
                theme_powerline_tokyo_night::quota_segment(),
            ],
            theme: "powerline-tokyo-night".to_string(),
        }
    }

    /// Create default theme configuration file with minimal template
    pub fn create_default_theme_file(theme_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let themes_dir = Self::get_themes_path();
        let theme_path = themes_dir.join(format!("{}.toml", theme_name));

        // Create themes directory if it doesn't exist
        std::fs::create_dir_all(&themes_dir)?;

        // Create a minimal template config
        let template_config = Self::get_default();
        let mut theme_config = template_config;
        theme_config.theme = theme_name.to_string();

        let toml_content = toml::to_string_pretty(&theme_config)?;

        // Add comments and examples to the template
        let template_content = format!(
            "# CCometixLine Theme Configuration: {}\n\
             # This file defines a custom theme for CCometixLine\n\
             # File location: ~/.claude/ccline/themes/{}.toml\n\
             # You can modify colors, icons, and styles below\n\
             \n\
             {}\n",
            theme_name,
            theme_name,
            toml_content.trim()
        );

        std::fs::write(&theme_path, template_content)?;
        Ok(())
    }

    /// Check if a theme exists (built-in or custom)
    pub fn theme_exists(theme_name: &str) -> bool {
        // Check built-in themes
        let built_in_themes = vec![
            "cometix",
            "default",
            "minimal",
            "gruvbox",
            "nord",
            "powerline-dark",
            "powerline-light",
            "powerline-rose-pine",
            "powerline-tokyo-night",
        ];

        if built_in_themes.contains(&theme_name) {
            return true;
        }

        // Check custom themes
        let themes_dir = Self::get_themes_path();
        let theme_path = themes_dir.join(format!("{}.toml", theme_name));
        theme_path.exists()
    }

    /// Delete a custom theme file
    pub fn delete_theme(theme_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Don't allow deleting built-in themes
        let built_in_themes = vec![
            "cometix",
            "default",
            "minimal",
            "gruvbox",
            "nord",
            "powerline-dark",
            "powerline-light",
            "powerline-rose-pine",
            "powerline-tokyo-night",
        ];

        if built_in_themes.contains(&theme_name) {
            return Err(format!("Cannot delete built-in theme: {}", theme_name).into());
        }

        let themes_dir = Self::get_themes_path();
        let theme_path = themes_dir.join(format!("{}.toml", theme_name));

        if theme_path.exists() {
            std::fs::remove_file(theme_path)?;
        }

        Ok(())
    }

    /// Copy an existing theme to create a new one
    pub fn copy_theme(
        source_theme: &str,
        new_theme: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let source_config = Self::get_theme(source_theme);
        Self::save_theme(new_theme, &source_config)?;
        Ok(())
    }

    /// Get theme type (built-in or custom)
    pub fn get_theme_type(theme_name: &str) -> &'static str {
        let built_in_themes = vec![
            "cometix",
            "default",
            "minimal",
            "gruvbox",
            "nord",
            "powerline-dark",
            "powerline-light",
            "powerline-rose-pine",
            "powerline-tokyo-night",
        ];

        if built_in_themes.contains(&theme_name) {
            "built-in"
        } else {
            "custom"
        }
    }
}
