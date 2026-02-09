use crate::client::openai::OpenAiClient;
use crate::client::prompt::BASIC_PROMPT;
use crate::utils::config::Config;
use crate::utils::diff;
use std::process::exit;

const LLVM_ERROR: &str = "Possible causes:

• API Key or URL configuration error
• Network connection issues(check your proxy)
• The model may be temporarily unstable, please try again later

Please check your configuration and try again.";

pub fn run_diff() {
    let config = Config::read();
    let open_ai_url = if let Some(url) = config.open_ai_url {
        url
    } else {
        eprintln!("URL is not set, please configure it. More info --help");
        exit(1);
    };
    let open_ai_key = if let Some(url) = config.open_ai_key {
        url
    } else {
        eprintln!("OpenAI key is not set, please configure it. More info --help");
        exit(1);
    };
    let lang = config.lang.unwrap_or("english".to_string());
    let prompt = BASIC_PROMPT.replace("{locale}", &lang);
    let client = OpenAiClient::new(&open_ai_url, &open_ai_key);
    let diff = diff::diff();
    match client.ask(&format!("{}\n\n{}", prompt, diff)) {
        Ok(response) => {
            println!("{}", response);
        }
        Err(_) => {
            println!("{}", LLVM_ERROR);
        }
    }
}
