use crate::tge::{renderer::Renderer, window::Window};

use crossterm::{
    cursor,
    terminal::{self, ClearType},
    ExecutableCommand,
};
use std::io::{self, Write};
use std::rc:Rc;

#[derive(Debug)]
pub struct Frame {
    pub content: Vec<String>,
}

pub struct Canvas {
    pub width: u16,
    pub height: u16,
    pub priority: u8,
    pub position: (u16, u16),
    pub windows: Vec<Window>, // 画布中的窗口
}

impl Canvas {
    pub fn new(width: u16, height: u16, priority: u8) -> Self {
        Self {
            width,
            height,
            priority,
            windows: Vec::new(),
        }
    }

    pub fn render(&self, renderer: &mut Renderer) {
        // 渲染画布中的所有窗口
        for window in &self.windows {
            renderer.queue_render(window); // 将窗口渲染任务加入到渲染队列
        }
    }
}
