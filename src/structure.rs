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

pub mod list;
pub mod tag;
