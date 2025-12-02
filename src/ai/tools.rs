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

    /// Tìm kiếm thực tế qua Google Custom Search JSON API
    /// Cần biến môi trường: GOOGLE_API_KEY, CX_ID
    pub async fn smart_search(&self, query: &str) -> Result<String, String> {
        let api_key = env::var("GOOGLE_API_KEY").unwrap_or_default();
        let cx = env::var("CX_ID").unwrap_or_default();
        
        if api_key.is_empty() {
            println!("⚠️ MISSING GOOGLE API KEY (Running in Offline Mode)");
            return Ok("Offline Mode: No Internet access configured.".to_string());
        }

        let url = format!(
            "https://www.googleapis.com/customsearch/v1?key={}&cx={}&q={}",
            api_key, cx, query
        );

        match self.client.get(&url).send().await {
            Ok(resp) => {
                if let Ok(json) = resp.json::<Value>().await {
                    // Lấy snippet đầu tiên
                    if let Some(items) = json.get("items").and_then(|i| i.as_array()) {
                        if let Some(first) = items.first() {
                            let snippet = first["snippet"].as_str().unwrap_or("");
                            return Ok(snippet.to_string());
                        }
                    }
                }
                Err("No results found".to_string())
            }
            Err(e) => Err(format!("Search Error: {}", e))
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

    /// Gọi OpenAI GPT-4o-mini hoặc Model tương đương
    pub async fn ask_ai(&self, prompt: &str) -> Result<String, String> {
        let api_key = env::var("OPENAI_API_KEY").unwrap_or_default();
        if api_key.is_empty() {
            return Ok("AI Offline: Please set OPENAI_API_KEY in .env".to_string());
        }

        let body = json!({
            "model": "gpt-4o-mini",
            "messages": [{"role": "user", "content": prompt}],
            "temperature": 0.7
        });

        let res = self.client.post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&body)
            .send()
            .await;

        match res {
            Ok(response) => {
                let json: Value = response.json().await.unwrap_or(json!({}));
                let content = json["choices"][0]["message"]["content"]
                    .as_str()
                    .unwrap_or("AI Silent");
                Ok(content.to_string())
            }
            Err(e) => Err(format!("LLM Error: {}", e))
        }
    }
}
