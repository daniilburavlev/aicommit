use crate::utils::fs::read_from_home_dir;
use std::collections::HashMap;

pub struct Prop {
    properties: HashMap<String, String>,
}

impl Prop {
    pub fn new(filename: &str) -> Self {
        if let Some(data) = read_from_home_dir(filename) {
            return Self {
                properties: Prop::parse(data),
            };
        }
        Self {
            properties: HashMap::new(),
        }
    }

    fn parse(data: String) -> HashMap<String, String> {
        let mut properties = HashMap::new();
        for line in data.lines() {
            let key_value: Vec<&str> = line.splitn(2, "=").collect();
            properties.insert(key_value[0].to_owned(), key_value[1].to_owned());
        }
        properties
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.properties.get(key).map(|value| value.to_owned())
    }
}
