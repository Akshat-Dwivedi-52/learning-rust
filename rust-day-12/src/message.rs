#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub sender: String,
    pub content: String,
}

impl ChatMessage {
    pub fn new(sender: &str, content: &str) -> Self {
        Self {
            sender: sender.to_string(),
            content: content.to_string(),
        }
    }
}