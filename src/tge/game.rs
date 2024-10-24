use crate::entity::Entity;
use crate::window::Window;
use crate::buffer::Buffer;
use crossterm::{event::{self, KeyCode, KeyEvent}};
use std::io::{self, Write};
use std::time::{Duration};

pub struct Game {
    pub running: bool,
    pub entities: Vec<Entity>,
    pub windows: Vec<Window>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            running: true,
            entities: Vec::new(),
            windows: Vec::new(),
        }
    }

    pub fn update(&mut self) {
    }

    pub fn render(&self, stdout: &mut dyn Write, vsync: bool) {
        let mut buffer = Buffer::new();

        buffer.add_line("\x1b[2J\x1b[H".to_string()); // ANSI Clean Screen
        buffer.add_line(format!("Press 'q' to quit"));

        for entity in &self.entities {
            entity.render(&mut buffer);
        }

        for window in &self.windows {
            window.render(&mut buffer);
        }

        if vsync {
            buffer.output_all(stdout);
        } else {
            buffer.output_one_by_one(stdout);
        }
    }

    pub fn handle_input(&mut self) {
        if event::poll(Duration::from_millis(100)).unwrap() {
            if let event::Event::Key(KeyEvent {
                code,
                ..
            }) = event::read().unwrap() {
                match code {
                    KeyCode::Char('q') => {
                        self.running = false;
                    }
                    _ => {}
                }
            }
        }
    }
}
