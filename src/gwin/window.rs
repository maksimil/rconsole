use crate::gwin::buffer::GBuffer;
use crate::term;
use std::mem;

pub struct Window {
    buff: GBuffer,
}

impl Window {
    pub fn new(size: (usize, usize)) -> Window {
        term::clear();
        Window {
            buff: GBuffer::new(size),
        }
    }

    pub fn push_buffer(&mut self, buff: GBuffer) {
        if self.buff.size() != buff.size() {
            panic!("Buffer sizes don't match");
        }

        let (width, _) = self.buff.size();

        for (i, (achars, aline, bchars)) in buff
            .buffer()
            .iter()
            .zip(self.buff.buffer().iter())
            .map(|(a, b)| {
                (
                    a.split("").collect::<Vec<_>>(),
                    a,
                    b.split("").collect::<Vec<_>>(),
                )
            })
            .enumerate()
        {
            let mut j = 0;
            while j < width {
                if achars[j] != bchars[j] {
                    let mut k = j + 1;
                    while k < width && achars[k] != bchars[k] {
                        k += 1;
                    }

                    term::moveto(((j - 1) as u16, i as u16));
                    term::putline(&aline[j - 1..k - 1]);

                    j = k - 1;
                }

                j += 1;
            }
        }

        self.buff = buff;
    }

    pub fn edit_buffer<F: FnOnce(GBuffer) -> GBuffer>(&mut self, edit: F) {
        self.push_buffer(edit(self.buff.clone()));
    }

    pub fn open_render_edit<'s>(&'s mut self) -> Render<'s> {
        let buffer = self.buff.clone();
        Render {
            window: self,
            buffer,
            cpos: (0, 0),
        }
    }

    pub fn open_render_clear<'s>(&'s mut self) -> Render<'s> {
        let bsize = self.buff.size();
        Render {
            window: self,
            buffer: GBuffer::new(bsize),
            cpos: (0, 0),
        }
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
        self.buffer.putline(self.cpos, s);
    }
}

impl<'s> Drop for Render<'s> {
    fn drop(&mut self) {
        self.window
            .push_buffer(mem::replace(&mut self.buffer, GBuffer::new((0, 0))));
    }
}
