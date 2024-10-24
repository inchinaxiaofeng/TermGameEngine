use std::fmt::Write;
use crate::buffer::Buffer;

#[derive()]
pub struct Entity {
    pub id: usize,
    pub name: String,
    pub position: (u16, u16),
}

impl Entity {
    pub fn render(&self, buffer: &mut Buffer) {
        buffer.add_line(format!("Entity: ID{}", self.id));
    }
}
