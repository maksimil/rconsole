use crate::render::{cpos, moveto};

pub mod list;
pub mod size_cell;
pub mod tag;

pub trait Element {
    fn size(&self) -> (u16, u16);

    fn render(&mut self) {
        let cp = cpos();
        self.render_inner();
        moveto(cp);
    }

    fn render_inner(&mut self);

    fn shouldupdate(&self) -> bool;
}

pub trait Navigate<T> {
    fn child_ref(&self, id: usize) -> Option<&T>;
    fn child_mut(&mut self, id: usize) -> Option<&mut T>;
}

pub trait SelfNavigate: Sized + Navigate<Self> {
    fn navigate_ref(&self, id: &[usize]) -> Option<&Self> {
        if id.len() == 0 {
            Some(self)
        } else {
            if let Some(child) = self.child_ref(id[0]) {
                child.navigate_ref(&id[1..])
            } else {
                None
            }
        }
    }

    fn navigate_mut(&mut self, id: &[usize]) -> Option<&mut Self> {
        if id.len() == 0 {
            Some(self)
        } else {
            if let Some(child) = self.child_mut(id[0]) {
                child.navigate_mut(&id[1..])
            } else {
                None
            }
        }
    }
}
