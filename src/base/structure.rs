use crate::base::gwindow::putline;

pub trait Structure {
    fn offset(&self) -> (u16, u16);
    fn size(&self) -> (u16, u16);
    fn render(&self);
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
        putline(self.text.as_str());
    }
}
