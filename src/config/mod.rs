use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub jira: JiraConfig,
    pub ui: UiConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JiraConfig {
    pub domain: String,
    pub username: String,
    pub api_token: String,
    pub default_board_id: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UiConfig {
    pub theme: String,
    pub refresh_interval: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            jira: JiraConfig {
                domain: "your-domain.atlassian.net".to_string(),
                username: "".to_string(),
                api_token: "".to_string(),
                default_board_id: None,
            },
            ui: UiConfig {
                theme: "default".to_string(),
                refresh_interval: 30,
            },
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        
        if !config_path.exists() {
            let default_config = Self::default();
            default_config.save()?;
            return Ok(default_config);
        }

        let content = std::fs::read_to_string(&config_path)?;
        let config: Config = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;
        
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(&config_path, content)?;
        Ok(())
    }

    fn config_path() -> Result<PathBuf> {
        let home = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE"))?;
        Ok(PathBuf::from(home).join(".config").join("jira-tui").join("config.json"))
    }
}
