use reqwest::{Client, Error};
use serde_json::json;

#[derive(Clone)]
pub struct OpenAIService {
    client: Client,
    api_key: String,
}

impl OpenAIService {
    pub fn new(api_key: String) -> Self {
        OpenAIService {
            client: Client::new(),
            api_key,
        }
    }

    pub async fn send_to_openai(&self, text: &str) -> Result<String, Error> {
        let url = "https://api.openai.com/v1/chat/completions";

        let response = self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&json!({
                "model": "gpt-3.5-turbo",
                "messages": [{ "role": "user", "content": text }],
                "max_tokens": 50,
                "temperature": 0.5,
            }))
            .send()
            .await?;

        let body = response.text().await?;
        Ok(body)
    }
}
