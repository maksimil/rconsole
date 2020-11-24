use crate::render::putline;

pub trait Element {
    fn size(&self) -> (u16, u16);
    fn render(&mut self);

    fn shouldupdate(&self) -> bool;
}

pub struct Tag {
    width: u16,
    text: String,
    shouldupdate: bool,
}

impl Tag {
    pub fn new(text: String) -> Tag {
        let width = text.len() as u16;
        Tag {
            text,
            width,
            shouldupdate: true,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn text_mut(&mut self) -> &mut String {
        self.shouldupdate = true;
        &mut self.text
    }
}

impl Element for Tag {
    fn size(&self) -> (u16, u16) {
        (self.width, 1)
    }

    fn render(&mut self) {
        self.shouldupdate = false;
        putline(self.text.as_str());
    }

    fn shouldupdate(&self) -> bool {
        self.shouldupdate
    }
}
