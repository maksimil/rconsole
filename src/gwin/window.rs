use crate::gwin::buffer::{GBuffer, PutLine, PutLineStr};
use crate::term;
use std::cmp::max;
use std::mem;
use std::ops::Drop;

pub struct Window {
    buff: GBuffer,
}

impl Window {
    pub fn new() -> Window {
        term::clear();
        term::ealt_screen();
        let (_, height) = term::size();
        Window {
            buff: GBuffer::new(height as usize),
        }
    }

    pub fn push_buffer(&mut self, mut buff: GBuffer) {
        {
            let height = max(self.buff.height(), buff.height());
            self.buff.expand(height);
            buff.expand(height);
        }

        for (y, (sline, bline)) in self
            .buff
            .lines()
            .iter()
            .zip(buff.lines().iter())
            .enumerate()
        {
            // merge indicies
            let mut i = 0;
            let mut j = 0;

            // editing places
            let mut sx = 0;
            let mut ex = 0;
            let mut spl: Vec<PutLineStr> = Vec::new();
            let mut bpl: Vec<PutLineStr> = Vec::new();
            while i < sline.len() || j < bline.len() {
                if i == sline.len() || bline[j].sx() < sline[i].sx() {
                    // next pl is bline[j]

                    j += 1;
                } else {
                    // next pl is sline[i]

                    i += 1;
                }
            }
        }
    }

    // pub fn edit_buffer<F: FnOnce(GBuffer) -> GBuffer>(&mut self, edit: F) {
    //     self.push_buffer(edit(self.buff.clone()));
    // }

    // pub fn open_render_edit<'s>(&'s mut self) -> Render<'s> {
    //     let buffer = self.buff.clone();
    //     Render {
    //         window: self,
    //         buffer,
    //         cpos: (0, 0),
    //     }
    // }

    // pub fn open_render_clear<'s>(&'s mut self) -> Render<'s> {
    //     let bsize = self.buff.size();
    //     Render {
    //         window: self,
    //         buffer: GBuffer::new(bsize),
    //         cpos: (0, 0),
    //     }
    // }
}

impl Drop for Window {
    fn drop(&mut self) {
        term::lalt_screen();
    }
}

pub struct Render<'s> {
    window: &'s mut Window,
    buffer: GBuffer,
    cpos: (usize, usize),
}

impl<'s> Render<'s> {
    pub fn moveto(&mut self, pos: (usize, usize)) {
        self.cpos = pos;
    }

    pub fn moveby(&mut self, (x, y): (usize, usize)) {
        self.cpos.0 += x;
        self.cpos.1 += y;
    }

    pub fn imoveby(&mut self, (x, y): (usize, usize)) {
        self.cpos.0 -= x;
        self.cpos.1 -= y;
    }
    pub fn putline(&mut self, s: &str) {
        self.buffer.putline_str(self.cpos, s);
    }
}

impl<'s> Drop for Render<'s> {
    fn drop(&mut self) {
        self.window
            .push_buffer(mem::replace(&mut self.buffer, GBuffer::new(0)));
    }
}
