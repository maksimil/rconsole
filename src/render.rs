use crossterm::{
    cursor::{
        position, MoveDown, MoveLeft, MoveRight, MoveTo, MoveUp, RestorePosition, SavePosition,
    },
    execute,
    style::Print,
    terminal::{Clear, ClearType},
};
use std::io::{stdout, Write};

const STDOUT_EXECUTION_ERR: &str = "Failed to use stdout";

pub fn clear() {
    execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0));
}

pub fn moveto(pos: (u16, u16)) {
    execute!(stdout(), MoveTo(pos.0, pos.1)).expect(STDOUT_EXECUTION_ERR);
}

pub fn moveby(offset: (u16, u16)) {
    execute!(stdout(), MoveRight(offset.0), MoveDown(offset.1)).expect(STDOUT_EXECUTION_ERR);
}

pub fn imoveby(offset: (u16, u16)) {
    execute!(stdout(), MoveLeft(offset.0), MoveUp(offset.1)).expect(STDOUT_EXECUTION_ERR);
}

pub fn putline(line: &str) {
    execute!(stdout(), SavePosition, Print(line), RestorePosition);
}

pub fn cpos() -> (u16, u16) {
    position().expect("Was not able to get cursor positioin")
}