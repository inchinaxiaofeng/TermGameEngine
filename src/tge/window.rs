pub struct Window {
    pub title: String,
    pub content: String, // 这里可以是文本或图像的表示
    pub position: (u16, u16),
    pub size: (u16, u16),
}

impl Window {
    pub fn new(title: &str, content: &str, position: (u16, u16), size: (u16, u16)) -> Self {
        Self {
            title: title.to_string(),
            content: content.to_string(),
            position,
            size,
        }
    }

    pub fn render(&self) -> String {
        // 渲染窗口内容的逻辑
        format!("{}: {}", self.title, self.content)
    }
}
