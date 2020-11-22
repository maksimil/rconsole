use crate::render::{moveby, putline};
use crate::structure::Structure;

pub struct Tag {
    pub offset: (u16, u16),
    pub size: (u16, u16),
    pub text: String,
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
