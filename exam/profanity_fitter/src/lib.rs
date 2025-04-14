pub struct Message {
    content: String,
    user: String,
}

impl Message {
    // تُنشئ رسالة جديدة
    pub fn new(ms: String, u: String) -> Message {
        Message {
            content: ms,
            user: u,
        }
    }

    // تُرسل الرسالة، إلا إذا كانت فارغة أو تحتوي على "stupid"
    pub fn send_ms(&self) -> Option<&str> {
        if self.content.is_empty() || self.content.to_lowercase().contains("stupid") {
            None
        } else {
            Some(&self.content)
        }
    }
}

// تفحص الرسالة وتعيد نتيجة حسب ما إذا كانت الرسالة مقبولة أو لا
pub fn check_ms(ms: &Message) -> (bool, &str) {
    match ms.send_ms() {
        Some(valid_content) => (true, valid_content),
        None => (false, "ERROR: illegal"),
    }
}
