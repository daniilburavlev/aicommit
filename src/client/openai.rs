use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct Message {
    pub content: String,
}

#[derive(Deserialize)]
struct Choice {
    pub message: Message,
}

#[derive(Deserialize)]
struct Response {
    pub choices: Vec<Choice>,
}

pub struct OpenAiClient<'a> {
    url: &'a str,
    token: &'a str,
    client: Client,
}

impl<'a> OpenAiClient<'a> {
    pub fn new(url: &'a str, token: &'a str) -> OpenAiClient<'a> {
        OpenAiClient {
            url,
            token,
            client: Client::new(),
        }
    }

    pub fn ask(&self, request: &str) -> Result<String, Box<dyn std::error::Error>> {
        let response = self
            .client
            .post(self.url)
            .header("Authorization", &format!("Bearer {}", self.token))
            .header("Content-Type", "application/json")
            .json(&json!({
                "temperature" : 1,
                "model" : "gpt-5-mini",
                "messages" : [
                    {
                        "role" : "user",
                        "content" : request
                    }
                ]
            }))
            .send()?;
        let response_text: Response = response.json()?;
        if let Some(choice) = response_text.choices.get(0) {
            Ok(choice.message.content.clone())
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Cannot get ").into())
        }
    }
}
