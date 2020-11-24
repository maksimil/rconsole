pub mod tag;

pub trait Element {
    fn size(&self) -> (u16, u16);
    fn render(&mut self);

    fn shouldupdate(&self) -> bool;
}
