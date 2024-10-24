pub struct Entity {
    pub id: usize,
    pub name: String,
    pub appearance: String, // 形象可以是字符或帧动画
}

impl Entity {
    pub fn new(id: usize, name: &str, appearance: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            appearance: appearance.to_string(),
        }
    }

    pub fn render(&self) -> String {
        // 渲染实体的逻辑
        format!("Entity: {} with appearance: {}", self.name, self.appearance)
    }
}
