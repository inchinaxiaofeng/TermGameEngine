/// Term Game Engine
use crossterm::{
    cursor,
    event::{self, KeyCode, KeyEvent},
    execute,
    terminal::{self, ClearType},
    ExecutableCommand,
};
use std::io::{self, Write};
use std::time::{Duration, Instant};
use std::{collections::HashMap, io::Stdout};
use std::{fmt::Write as FmtWrite, io::stdout};

struct Game {
    running: bool,
    entities: Vec<Entity>,
    windows: Vec<Window>,
}

struct Entity {
    id: usize,
    name: String,
    position: (u16, u16),
}

struct Window {
    title: String,
    position: (u16, u16),
    size: (u16, u16),
    visible: bool,
}

impl Game {
    fn new() -> Self {
        Self {
            running: true,
            entities: Vec::new(),
            windows: Vec::new(),
        }
    }

    fn update(&mut self) {
        // update status
    }

    fn add_entity(&mut self, name: &str, position: (u16, u16)) {
        let id = self.entities.len();
        self.entities.push(Entity {
            id,
            name: name.to_string(),
            position,
        });
    }

    fn add_window(&mut self, title: &str, position: (u16, u16), size: (u16, u16)) {
        self.windows.push(Window {
            title: title.to_string(),
            position,
            size,
            visible: true,
        });
    }

    fn render(&self, stdout: &mut io::Stdout) {
        stdout.execute(terminal::Clear(ClearType::All)).unwrap(); // Clean Screen
        stdout.execute(cursor::MoveTo(0, 0)).unwrap(); // move cursor to starter

        // render entity
        for entity in &self.entities {
            writeln!(
                stdout,
                "Entity: ID: {}, Name: {}, Position: {:?}",
                entity.id, entity.name, entity.position
            )
            .unwrap();
        }

        // render windows
        for window in &self.windows {
            if window.visible {
                self.render_window(stdout, window);
            }
        }

        writeln!(stdout, "Welcome to the Terminal Game Engine").unwrap();
        writeln!(stdout, "Press 'q' to quit.").unwrap();
        writeln!(stdout, "Press 'w' to toggle window.").unwrap();

        stdout.flush().unwrap();
    }

    fn render_window(&self, stdout: &mut io::Stdout, window: &Window) {
        let (x, y) = window.position;
        let (width, height) = window.size;

        // render the edge of window
        let border_top = format!(
            "{}+{}+",
            " ".repeat(x as usize),
            "-".repeat((width - 2) as usize)
        );
        let title_line = format!("{}| {} |", " ".repeat(x as usize), window.title);
        let border_bottom = format!(
            "{}+{}+",
            " ".repeat(x as usize),
            "-".repeat((width - 2) as usize)
        );

        // Output the edge and title
        stdout.execute(cursor::MoveTo(x, y)).unwrap();
        writeln!(stdout, "{}", border_top).unwrap();
        stdout.execute(cursor::MoveTo(x, y + 1)).unwrap();
        writeln!(stdout, "{}", title_line).unwrap();
        stdout.execute(cursor::MoveTo(x, y + 2)).unwrap();
        writeln!(stdout, "{}", border_bottom).unwrap();

        // render the text of window
        for i in 0..(height - 3) {
            stdout.execute(cursor::MoveTo(x, y + 3 + i as u16)).unwrap();
            writeln!(
                stdout,
                "{}|{}|",
                " ".repeat(x as usize),
                " ".repeat((width - 2) as usize)
            )
            .unwrap();
        }

        stdout
            .execute(cursor::MoveTo(x, y + 3 + (height - 3) as u16))
            .unwrap();
        writeln!(
            stdout,
            "{}+{}+",
            " ".repeat(x as usize),
            "-".repeat((width - 2) as usize)
        )
        .unwrap();
    }

    fn handle_input(&mut self) {
        if event::poll(Duration::from_millis(100)).unwrap() {
            if let event::Event::Key(KeyEvent {
                code,
                modifiers,
                kind,
                state,
            }) = event::read().unwrap()
            {
                match code {
                    KeyCode::Char('q') => {
                        self.running = false;
                    }
                    KeyCode::Char('w') => {
                        if let Some(window) = self.windows.get_mut(0) {
                            window.visible = !window.visible;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

fn main() {
    let mut stdout = io::stdout();
    terminal::enable_raw_mode().unwrap();

    let mut game = Game::new();
    let mut last_frame = Instant::now();

    game.add_entity("Player", (5, 5));
    game.add_window("Settings", (2, 2), (30, 10));

    while game.running {
        game.handle_input();
        game.update();
        game.render(&mut stdout);

        // control frame freq
        let frame_time = Duration::from_millis(100); // 10FPS
        if last_frame.elapsed() < frame_time {
            std::thread::sleep(frame_time - last_frame.elapsed());
        }
        last_frame = Instant::now();
    }

    terminal::disable_raw_mode().unwrap();
}
