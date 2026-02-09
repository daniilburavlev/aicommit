use serde::{Deserialize, Serialize};

use std::process::exit;

use crate::utils::fs::{get_home_path, read_data, write_data};

pub const OPENAI_DEFAULT_URL: &str = "https://api.openai.com/v1/chat/completions";

const DEFAULT_PATH: &str = ".config/aicommit/config.json";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Config {
    pub open_ai_url: Option<String>,
    pub open_ai_key: Option<String>,
}

impl Config {
    pub fn read() -> Config {
        let mut saved = Self::read_from_disk().unwrap_or_default();
        if saved.open_ai_url.is_none() {
            saved.open_ai_url = Some(OPENAI_DEFAULT_URL.to_string());
        }
        saved
    }

    pub fn save(&self) {
        let mut config = Config::read();
        if let Some(open_ai_url) = self.open_ai_url.clone() {
            config.open_ai_url = Some(open_ai_url);
        }
        if let Some(open_ai_key) = self.open_ai_key.clone() {
            config.open_ai_key = Some(open_ai_key);
        }
        let Ok(json) = serde_json::to_string_pretty(&config) else {
            return;
        };
        if !write_data(&get_path(DEFAULT_PATH), &json) {
            eprintln!("Failed saving config");
            exit(1);
        }
    }

    fn read_from_disk() -> Option<Config> {
        let path = get_path(DEFAULT_PATH);
        let config = read_data(&path)?;
        let config = serde_json::from_str(&config).ok()?;
        Some(config)
    }
}

fn get_path(path: &str) -> String {
    match get_home_path(path) {
        Some(path) => path,
        None => {
            eprintln!("cannot load config, invalid path");
            exit(-1);
        }
    }
}
