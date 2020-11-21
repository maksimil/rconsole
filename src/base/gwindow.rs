use crate::base::structure::Structure;
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

pub fn lf(times: u16) {
    for _ in 0..times {
        execute!(stdout(), Print("\n")).expect(STDOUT_EXECUTION_ERR);
    }
}

pub fn prerender(height: u16) {
    lf(height);
    imoveby((0, height));
}

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

#[derive(Clone, Copy)]
pub enum NextPos {
    Right,
    Bottom,
}

use NextPos::*;

pub fn render_struct<T: Structure>(structure: &T, np: NextPos) {
    let offset = structure.offset();
    let size = structure.size();
    let pos = position().expect(STDOUT_EXECUTION_ERR);
    moveby(offset);
    structure.render();
    moveto(pos);
    match np {
        Right => {
            moveby((size.0 + offset.0, 0));
        }
        Bottom => {
            moveby((0, size.1 + offset.1));
        }
    };
}
