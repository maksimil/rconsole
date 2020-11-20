use crossterm::{execute, style::Print};
use std::io::{Stdout, Write};

pub trait Structure {
    fn position(&self) -> (u16, u16);
    fn render(&self, stdout: &mut Stdout);
}

pub struct Tag {
    pub text: String,
    pub pos: (u16, u16),
}

impl Structure for Tag {
    fn position(&self) -> (u16, u16) {
        self.pos
    }

    fn render(&self, stdout: &mut Stdout) {
        execute!(stdout, Print(self.text.as_str()));
    }
}
