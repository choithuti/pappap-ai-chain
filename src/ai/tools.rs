// src/ai/tools.rs
pub struct Oracle;
impl Oracle {
    pub fn new() -> Self { Self }
    pub async fn smart_search(&self, _q: &str) -> Result<String, String> {
        Ok("Không tìm thấy".to_string()) // Mock
    }
}

pub struct LLMBridge;
impl LLMBridge {
    pub fn new() -> Self { Self }
    pub async fn ask_ai(&self, _q: &str) -> Result<String, String> {
        Ok("I am Pappap AI".to_string()) // Mock
    }
}
