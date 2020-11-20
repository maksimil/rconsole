use crate::base::structure::Structure;
use crossterm::{
    cursor::{position, MoveRight, MoveToColumn},
    execute,
    style::Print,
};
use std::io::{Stdout, Write};

fn lf(stdout: &mut Stdout, times: u16) {
    for _ in 0..times {
        execute!(stdout, Print("\n"));
    }
}

pub fn render_struct<T: Structure>(stdout: &mut Stdout, s: T) {
    let (col, _) = position().expect("Failed to get cursor positioin");
    let (top, left) = s.position();
    lf(stdout, top);
    execute!(stdout, MoveRight(left));
    s.render(stdout);
    execute!(stdout, Print("\n"), MoveToColumn(col));
}
