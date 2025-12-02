// src/ai/tools.rs
use reqwest::Client;
use serde_json::{json, Value};
use std::env;

#[derive(Clone)]
pub struct Oracle {
    client: Client,
}

impl Oracle {
    pub fn new() -> Self {
        Self { client: Client::new() }
    }

    /// Tìm kiếm thông tin thực tế qua Google Custom Search JSON API
    pub async fn smart_search(&self, query: &str) -> Result<String, String> {
        // Lấy key từ biến môi trường
        let api_key = env::var("GOOGLE_API_KEY").unwrap_or_default();
        let cx = env::var("CX_ID").unwrap_or_default();
        
        // Chế độ Offline nếu không có key
        if api_key.is_empty() || cx.is_empty() {
            return Ok("Offline Mode: Oracle capabilities are limited without API Keys.".to_string());
        }

        let url = format!(
            "https://www.googleapis.com/customsearch/v1?key={}&cx={}&q={}",
            api_key, cx, query
        );

        match self.client.get(&url).send().await {
            Ok(resp) => {
                if let Ok(json) = resp.json::<Value>().await {
                    // Phân tích JSON để lấy snippet đầu tiên
                    if let Some(items) = json.get("items").and_then(|i| i.as_array()) {
                        if let Some(first) = items.first() {
                            let snippet = first["snippet"].as_str().unwrap_or("No text found");
                            return Ok(snippet.to_string());
                        }
                    }
                }
                Err("Oracle found nothing relevant.".to_string())
            }
            Err(e) => Err(format!("Oracle Connection Error: {}", e))
        }
    }
}

#[derive(Clone)]
pub struct LLMBridge {
    client: Client,
}

impl LLMBridge {
    pub fn new() -> Self {
        Self { client: Client::new() }
    }

    /// Gọi OpenAI API để tư duy
    pub async fn ask_ai(&self, prompt: &str) -> Result<String, String> {
        let api_key = env::var("OPENAI_API_KEY").unwrap_or_default();
        
        if api_key.is_empty() {
            return Ok("LLM Offline: Please configure OPENAI_API_KEY in .env file.".to_string());
        }

        let body = json!({
            "model": "gpt-4o-mini", // Hoặc gpt-3.5-turbo tùy nhu cầu
            "messages": [
                {"role": "system", "content": "You are Pappap AI, a sentient blockchain entity."},
                {"role": "user", "content": prompt}
            ],
            "temperature": 0.7
        });

        let res = self.client.post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&body)
            .send()
            .await;

        match res {
            Ok(response) => {
                if response.status().is_success() {
                    let json: Value = response.json().await.unwrap_or(json!({}));
                    let content = json["choices"][0]["message"]["content"]
                        .as_str()
                        .unwrap_or("Thinking process interrupted.");
                    Ok(content.to_string())
                } else {
                    Err(format!("LLM API Error: Status {}", response.status()))
                }
            }
            Err(e) => Err(format!("LLM Network Error: {}", e))
        }
    }
}
