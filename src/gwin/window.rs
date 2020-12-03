use crate::drawable::Drawable;
use crate::gwin::buffer::{enbuffer, GBuffer};
use crate::term;
use std::cmp::max;
use std::iter;
use std::mem;
use std::ops::Drop;

pub struct Window {
    buff: GBuffer,
}

pub fn commit_to_screen(prev: &str, next: &str, pos: (usize, usize)) {
    let mut pchars = prev.split("");
    pchars.next();
    let mut nchars = next.split("");
    nchars.next();

    for (x, (pchar, nchar)) in pchars.zip(nchars).enumerate() {
        if pchar != nchar {
            term::moveto((pos.0 as u16 + x as u16, pos.1 as u16));
            term::putline(nchar);
        }
    }
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
            let mut sstring: String = String::new();
            let mut bstring: String = String::new();
            while i < sline.len() || j < bline.len() {
                let (isspl, pl) =
                    if i == sline.len() || (j < bline.len() && bline[j].sx() < sline[i].sx()) {
                        // next pl is bline[j]
                        j += 1;
                        (false, &bline[j - 1])
                    } else {
                        // next pl is sline[i]
                        i += 1;
                        (true, &sline[i - 1])
                    };

                if pl.sx() >= ex {
                    // reisize strings
                    let slen = max(sstring.len(), bstring.len());
                    bstring.extend(iter::repeat(' ').take(slen - bstring.len()));
                    sstring.extend(iter::repeat(' ').take(slen - sstring.len()));

                    // apped changes to screen
                    commit_to_screen(&sstring, &bstring, (sx, y));

                    // restore state
                    sx = pl.sx();
                    ex = pl.ex();
                    sstring.clear();
                    bstring.clear();
                }
                // buffer changes
                enbuffer(
                    if isspl { &mut sstring } else { &mut bstring },
                    &pl.offset(-(sx as isize)),
                );
                ex = max(ex, pl.ex());
            }
            // reisize strings
            let slen = max(sstring.len(), bstring.len());
            bstring.extend(iter::repeat(' ').take(slen - bstring.len()));
            sstring.extend(iter::repeat(' ').take(slen - sstring.len()));

            // apped changes to screen
            commit_to_screen(&sstring, &bstring, (sx, y));
        }
        mem::swap(&mut self.buff, &mut buff);
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

    pub fn open_render_clear<'s>(&'s mut self) -> Render<'s> {
        let bsize = self.buff.height();
        Render {
            window: self,
            buffer: GBuffer::new(bsize),
            cpos: (0, 0),
        }
    }

    pub fn push_drawable<D: Drawable>(&mut self, drawable: D) {
        let mut r = self.open_render_clear();
        drawable.draw(&mut r);
    }
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
