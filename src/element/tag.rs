use crate::element::{Element, Navigate};
use crate::render::putline;

pub struct Tag {
    text: String,
    shouldupdate: bool,
}

impl Tag {
    pub fn new(text: String) -> Tag {
        Tag {
            text,
            shouldupdate: true,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }
    pub fn set_text(&mut self, s: &str) {
        self.shouldupdate = true;
        self.text = String::from(s);
    }
}

impl Element for Tag {
    fn size(&self) -> (u16, u16) {
        (self.text.len() as u16, 1)
    }

    fn render_inner(&mut self) {
        self.shouldupdate = false;
        putline(self.text.as_str());
    }

    fn shouldupdate(&self) -> bool {
        self.shouldupdate
    }
}

impl<T> Navigate<T> for Tag {
    fn child_ref(&self, _: usize) -> Option<&T> {
        None
    }

    fn child_mut(&mut self, _: usize) -> Option<&mut T> {
        None
    }
}
