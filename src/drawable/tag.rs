use crate::drawable::Drawable;
use crate::gwin::window::Render;

pub struct Tag<S: AsRef<str>> {
    text: S,
}

pub type TagStr<'s> = Tag<&'s str>;

impl<S: AsRef<str>> Tag<S> {
    pub fn new(text: S) -> Tag<S> {
        Tag { text }
    }
}

impl<S: AsRef<str>> Drawable for Tag<S> {
    fn size(&self) -> (usize, usize) {
        (self.text.as_ref().len(), 1)
    }

    fn draw(&self, r: &mut Render) {
        r.putline(self.text.as_ref());
    }
}
