use std::io::{self, Write};

pub struct Entity {
    pub id: usize,
    pub name: String,
    pub position: (u16, u16),
    traits: Vec<Box<dyn EntityTrait>>,
}

impl Entity {
    pub fn new(id: usize, name: &str, position: (u16, u16)) -> Self {
        Entity {
            id,
            name: name.to_string(),
            position,
            traits: Vec::new(),
        }
    }

    pub fn with_trait(mut self, traits: Vec<Box<dyn EntityTrait>>) -> Self {
        self.traits.extend(traits);
        self
    }

    pub fn render(&self) -> String {
        // 渲染实体的逻辑
        for theTrait in &self.traits {
            theTrait.apply();
        }
    }
}
