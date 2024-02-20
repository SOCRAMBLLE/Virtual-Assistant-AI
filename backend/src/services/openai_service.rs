use dotenv::dotenv;
use reqwest::{Client, Error};
use serde_json::json;
use std::env;

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
        dotenv().ok();
        let api_key = env::var("OPENAI_KEY").expect("OPENAI_KEY must be set".into());

        let url = "https://api.openai.com/v1/chat/completions";
        // let openai_api_key = "apikey";

        let response = self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&json!({
                "model": "gpt-3.5-turbo",
                "messages": [{ "role": "user", "content": text }],
                "max_tokens": 50,  // Configure conforme necessário
                "temperature": 0.5,
            }))
            .send()
            .await?;

        let body = response.text().await?;
        Ok(body)
    }
}

// pub async fn ask(&self, prompt: &str) -> Result<String, Error> {
//     let response = self
//         .client
//         .post("https://api.openai.com/v4/completions")
//         .header("Authorization", format!("Bearer {}", self.api_key))
//         .json(&serde_json::json!({
//             "model": "text-davinci-003",
//             "prompt": prompt,
//             "temperature": 0.5,
//             "max_tokens": 100,
//         }))
//         .send()
//         .await?;

//     let response_body = response.text().await?;
//     Ok(response_body)
// }
// let openai_api_key = "APIKEY";
//     let url = "https://api.openai.com/v1/chat/completions";

//     pub async fn send_to_openai(&self, text: &str) -> Result<String, reqwest::Error> {
//         let openai_api_key = "APIKEY";
//         let url = "https://api.openai.com/v1/chat/completions";

//         let client = reqwest::Client::new();
//         let response = self
//             .client
//             .post(url)
//             .header("Content-Type", "application/json")
//             .header("Authorization", format!("Bearer {}", openai_api_key))
//             .json(&json!({
//                 "temperature": 0.5,
//                 "model": "gpt-3.5-turbo",
//                 "prompt": text,
//                 "max_tokens": 50,  // Configure conforme necessário
//             }))
//             .send()
//             .await?;

//         let body = response.text().await?;
//         Ok(body)
//     }
// }
