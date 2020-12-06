use crate::gwin::window::Render;

pub mod list;
pub mod tag;

pub trait Drawable {
    fn size(&self) -> (usize, usize);
    fn draw(&self, r: &mut Render);
}

pub enum StdDrawable {
    Tag(tag::Tag<String>),
    List(list::List<StdDrawable>),
}

impl Drawable for StdDrawable {
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

pub fn std_tag(string: String) -> StdDrawable {
    StdDrawable::Tag(tag::Tag::new(string))
}

pub fn std_list(list: list::List<StdDrawable>) -> StdDrawable {
    StdDrawable::List(list)
}
