use std::collections::HashMap;
use std::env::home_dir;
use std::fs::OpenOptions;
use std::io::Read;
use std::process::exit;

pub struct Prop {
    properties: HashMap<String, String>,
}

impl Prop {
    pub fn new(filename: &str) -> Self {
        if let Some(mut home_dir) = home_dir() {
            home_dir.push(filename);
            if let Ok(mut file) = OpenOptions::new()
                .create(true)
                .read(true)
                .write(true)
                .open(home_dir)
            {
                let mut properties = String::new();
                if let Ok(_) = file.read_to_string(&mut properties) {
                    let properties = Prop::parse(properties);
                    Self { properties }
                } else {
                    eprintln!("Cannot read properties file");
                    exit(1);
                }
            } else {
                eprintln!("Cannot open properties file");
                exit(1);
            }
        } else {
            exit(1);
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
        if let Some(value) = self.properties.get(key) {
            Some(value.to_owned())
        } else {
            None
        }
    }
}
