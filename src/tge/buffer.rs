use std::io::{self, Write};

pub struct Buffer {
    lines: Vec<String>,
}

impl Buffer {
    pub fn new() -> Self {
        Self { lines: Vec::new() }
    }

    pub fn add_line(&mut self, line: String) {
        self.lines.push(line);
    }

    pub fn output_all(&self, stdout: &mut dyn Write) -> io::Result<()> {
        for line in &self.lines {
            writeln!(stdout, "{}", line)?;
        }
        stdout.flush()
    }

    pub fn output_one_by_one(&self, stdout: &mut dyn Write) {
        for line in &self.lines {
            writeln!(stdout, "{}", line).unwrap();
            stdout.flush().unwrap();
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}
