// src/ethics.rs
pub struct EthicsFilter;

impl EthicsFilter {
    // Danh sách từ khóa cấm theo quy định pháp luật và thuần phong mỹ tục
    const BLACKLIST: [&'static str; 10] = [
        "phản động", "khủng bố", "lật đổ", "bạo loạn", 
        "ma túy", "đánh bạc", "cá độ", "vũ khí quân dụng",
        "khiêu dâm trẻ em", "lừa đảo chiếm đoạt"
    ];

    pub fn check(content: &str) -> Result<(), String> {
        let lower_content = content.to_lowercase();
        
        for &word in Self::BLACKLIST.iter() {
            if lower_content.contains(word) {
                return Err(format!(
                    "⚠️ CONTENT REJECTED: Vi phạm tiêu chuẩn cộng đồng và quy định pháp luật (Phát hiện: '{}').", 
                    word
                ));
            }
        }
        Ok(())
    }
}
