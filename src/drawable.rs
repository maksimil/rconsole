use crate::gwin::window::Render;

pub mod list;
pub mod tag;

pub trait Drawable {
    fn size(&self) -> (usize, usize);
    fn draw(&self, r: &mut Render);
}

pub enum StdDrawable<'s> {
    Tag(tag::TagStr<'s>),
    List(list::List<StdDrawable<'s>>),
}

impl<'s> Drawable for StdDrawable<'s> {
    fn size(&self) -> (usize, usize) {
        match self {
            Self::Tag(s) => s.size(),
            Self::List(s) => s.size(),
        }
    }

    fn draw(&self, r: &mut Render) {
        match self {
            Self::Tag(s) => s.draw(r),
            Self::List(s) => s.draw(r),
        }
    }
}
