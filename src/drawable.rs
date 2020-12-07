use crate::gwin::window::Render;

pub mod list;
pub mod tag;

pub trait Drawable {
    fn size(&self) -> (usize, usize);
    fn draw(&self, r: &mut Render);
}

impl Drawable for Box<dyn Drawable> {
    fn size(&self) -> (usize, usize) {
        self.as_ref().size()
    }

    fn draw(&self, r: &mut Render) {
        self.as_ref().draw(r);
    }
}
