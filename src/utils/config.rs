use crate::utils::common::Prop;
use std::fs;
use std::process::exit;

pub const OPENAI_URL: &str = "OPENAI_URL";
pub const OPENAI_KEY: &str = "OPENAI_KEY";

const DEFAULT_PATH: &str = ".config";

#[derive(Debug)]
pub struct Config {
    pub open_ai_url: Option<String>,
    pub open_ai_key: Option<String>,
}

impl Config {
    pub fn read() -> Config {
        let properties = Prop::new(DEFAULT_PATH);
        let open_ai_url = properties.get(OPENAI_URL);
        let open_ai_key = properties.get(OPENAI_KEY);
        Config {
            open_ai_url,
            open_ai_key,
        }
    }

    pub fn save(&self) {
        let mut config = Config::read();
        if let Some(open_ai_url) = self.open_ai_url.clone() {
            config.open_ai_url = Some(open_ai_url);
        }
        if let Some(open_ai_key) = self.open_ai_key.clone() {
            config.open_ai_key = Some(open_ai_key);
        }
        let properties = format!(
            "{}={}\n{}={}",
            OPENAI_URL,
            config.open_ai_url.unwrap(),
            OPENAI_KEY,
            config.open_ai_key.unwrap()
        );
        if let Err(_) = fs::write(DEFAULT_PATH, properties) {
            eprintln!("Failed saving config");
            exit(1);
        }
    }
}
