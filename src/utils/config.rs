use serde::{Deserialize, Serialize};

use crate::utils::fs::{read_from_home_dir, write_to_home_dir};
use std::{collections::HashMap, process::exit};

pub const OPENAI_DEFAULT_URL: &str = "https://api.openai.com/v1/chat/completions";
pub const DEFAULT_LANG: &str = "english";

const DEFAULT_PATH: &str = ".aicommit";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Config {
    #[serde(rename = "OPENAI_URL")]
    pub open_ai_url: Option<String>,
    #[serde(rename = "OPENAI_KEY")]
    pub open_ai_key: Option<String>,
    pub lang: Option<String>,
}

impl Config {
    pub fn read() -> Config {
        let mut saved = Self::read_from_disk(DEFAULT_PATH).unwrap_or_default();
        if saved.open_ai_url.is_none() {
            saved.open_ai_url = Some(OPENAI_DEFAULT_URL.to_string());
        }
        if saved.lang.is_none() {
            saved.lang = Some(DEFAULT_LANG.to_string())
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
        if let Some(lang) = self.lang.clone() {
            config.lang = Some(lang);
        }
        let Ok(json) = serde_json::to_value(&config) else {
            return;
        };
        let Ok(json) = serde_json::from_value::<HashMap<String, String>>(json) else {
            return;
        };
        let mut properties = String::new();
        for (key, value) in json {
            properties.push_str(&key);
            properties.push('=');
            properties.push_str(&value);
            properties.push_str("\r\n");
        }
        if !write_to_home_dir(DEFAULT_PATH, &properties) {
            eprintln!("Failed saving config");
            exit(1);
        }
    }

    fn read_from_disk(path: &str) -> Option<Config> {
        let props = read_from_home_dir(path)?;
        let mut properties = HashMap::new();
        for line in props.lines() {
            let key_value: Vec<&str> = line.splitn(2, "=").collect();
            properties.insert(key_value[0].to_owned(), key_value[1].to_owned());
        }
        let json = serde_json::to_value(&properties).ok()?;
        let config = serde_json::from_value(json).ok()?;
        Some(config)
    }
}
