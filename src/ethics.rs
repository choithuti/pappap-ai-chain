// src/ethics.rs
pub struct EthicsFilter;

impl EthicsFilter {
    // Danh sách từ khóa cấm theo quy định (Demo rút gọn)
    const BLACKLIST: [&'static str; 10] = [
        "phản động", "khủng bố", "lật đổ", "bạo loạn", 
        "ma túy", "đánh bạc", "cá độ", "vũ khí",
        "khiêu dâm", "lừa đảo"
    ];

    pub fn check(content: &str) -> Result<(), String> {
        let lower_content = content.to_lowercase();
        
        for &word in Self::BLACKLIST.iter() {
            if lower_content.contains(word) {
                // Vi phạm -> Trả về lỗi ngay lập tức
                return Err(format!(
                    "⚠️ BLOCK BLOCKED: Vi phạm quy tắc an toàn thông tin & Luật An ninh mạng (Phát hiện từ khóa: '{}').", 
                    word
                ));
            }
        }
        Ok(())
    }
}
