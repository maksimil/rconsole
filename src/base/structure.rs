use crate::base::gwindow::{moveby, putline, render_struct, NextPos, NextPos::*};
use std::cmp::max;
use std::ops::Deref;

pub trait Structure {
    fn offset(&self) -> (u16, u16);
    fn size(&self) -> (u16, u16);
    fn render(&self);
}

impl Structure for Box<dyn Structure> {
    fn offset(&self) -> (u16, u16) {
        self.deref().offset()
    }

    fn size(&self) -> (u16, u16) {
        self.deref().size()
    }

    fn render(&self) {
        self.deref().render();
    }
}

pub struct Tag {
    offset: (u16, u16),
    size: (u16, u16),
    text: String,
}

impl Tag {
    pub fn new(text: String, offset: (u16, u16), width: u16) -> Tag {
        let len = text.len() as u16;
        Tag {
            text,
            offset,
            size: if width == 0 {
                (len, 1)
            } else {
                (width, len / width + 1)
            },
        }
    }
}

impl Structure for Tag {
    fn offset(&self) -> (u16, u16) {
        self.offset
    }

    fn size(&self) -> (u16, u16) {
        self.size
    }

    fn render(&self) {
        let (width, height) = (self.size.0 as usize, self.size.1 as usize);
        let s = if width * height >= self.text.len() {
            &self.text[..]
        } else {
            &self.text[0..width * height]
        };

        let mut i = 0;
        while i + width < s.len() {
            putline(&s[i..i + width]);
            moveby((0, 1));
            i += width;
        }

        putline(&s[i..s.len()]);
    }
}

pub struct List {
    offset: (u16, u16),
    chained: NextPos,
    children: Vec<Box<dyn Structure>>,
}

impl List {
    pub fn new(chained: NextPos, offset: (u16, u16)) -> List {
        List {
            offset,
            chained,
            children: Vec::new(),
        }
    }

    pub fn push<T: Structure + 'static>(&mut self, e: T) {
        self.children.push(Box::new(e));
    }
}

impl Structure for List {
    fn offset(&self) -> (u16, u16) {
        self.offset
    }

    fn size(&self) -> (u16, u16) {
        let mut s = (0, 0);
        match self.chained {
            Right => {
                for child in self.children.iter() {
                    let (cw, ch) = child.size();
                    let (ow, oh) = child.offset();
                    s.0 += cw + ow;
                    s.1 = max(s.1, ch + oh);
                }
            }
            Bottom => {
                for child in self.children.iter() {
                    let (cw, ch) = child.size();
                    let (ow, oh) = child.offset();
                    s.0 = max(s.0, cw + ow);
                    s.1 += ch + oh;
                }
            }
        }
        s
    }

    fn render(&self) {
        for child in self.children.iter() {
            render_struct(child, self.chained);
        }
    }
}
