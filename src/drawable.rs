use crate::gwin::window::Render;

pub mod tag;

pub trait Drawable {
    fn size(&self) -> (usize, usize);
    fn draw(&self, r: &mut Render);
}
