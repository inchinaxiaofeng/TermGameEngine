use std::fmt::Write;
use crate::buffer::Buffer;

#[derive()]
pub  struct Window {
    pub title: String,
    pub position: (u16, u16),
    pub size: (u16, u16),
    pub visible: bool,
}

impl Window {
    pub fn render(&self, buffer: &mut Buffer) {
        if self.visible {
            let (x,y) = self.position;
            let (width, height) = self.size;
            let border_top = format!("{}+{}+", " ".repeat(x as usize), "-".repeat((width-2) as usize));
            let title_line = format!("{}| {} |", " ".repeat(x as usize), self.title);
            let border_bottom = format!("{}+{}+", " ".repeat(x as usize), "-".repeat((width-2) as usize));

            buffer.add_line(border_top);
            buffer.add_line(title_line);
            buffer.add_line(border_bottom);

            for _ in 0..(height - 3) {
                buffer.add_line(format!("{}|{}|", " ".repeat(x as usize), " ".repeat((width-2) as usize)));
            }
            buffer.add_line(format!("{}+{}+", " ".repeat(x as usize), "-".repeat((width-2) as usize)));
        }
    }
}
