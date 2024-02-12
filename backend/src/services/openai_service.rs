use reqwest::{Client, Error};

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

    pub async fn ask(&self, prompt: &str) -> Result<String, Error> {
        let response = self
            .client
            .post("https://api.openai.com/v4/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&serde_json::json!({
                "model": "text-davinci-003",
                "prompt": prompt,
                "temperature": 0.5,
                "max_tokens": 100,
            }))
            .send()
            .await?;

        let response_body = response.text().await?;
        Ok(response_body)
    }
}
