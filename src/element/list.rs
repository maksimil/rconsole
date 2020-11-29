use crate::element::{size_cell::SizeCell, Element, Navigate};
use crate::render::moveby;
use std::cmp::max;

#[derive(Clone, Copy)]
pub enum NextPos {
    Bottom,
    Right,
}

use NextPos::*;

pub struct List<E: Element> {
    elements: Vec<SizeCell<E>>,
    chain: NextPos,
    shouldupdate: bool,
}

impl<E: Element> List<E> {
    pub fn new(chain: NextPos) -> List<E> {
        List {
            chain,
            shouldupdate: true,
            elements: Vec::new(),
        }
    }

    pub fn push(&mut self, element: E) -> usize {
        self.shouldupdate = true;
        self.elements.push(SizeCell::new(element));
        self.elements.len() - 1
    }
}

impl<E: Element> Element for List<E> {
    fn size(&self) -> (u16, u16) {
        let mut s = (0, 0);
        for element in self.elements.iter() {
            let size = element.size();
            match self.chain {
                Right => {
                    s.0 += size.0;
                    s.1 = max(s.1, size.1);
                }
                Bottom => {
                    s.0 = max(s.0, size.0);
                    s.1 += size.1;
                }
            }
        }
        s
    }

    fn render_inner(&mut self) {
        self.shouldupdate = false;
        for element in self.elements.iter_mut() {
            if element.shouldupdate() {
                element.render();
            }

            match self.chain {
                Right => {
                    moveby((element.size().0, 0));
                }
                Bottom => {
                    moveby((0, element.size().1));
                }
            }
        }
    }

    fn shouldupdate(&self) -> bool {
        self.shouldupdate
    }
}

impl<E: Element> Navigate<E> for List<E> {
    fn child_ref(&self, id: usize) -> Option<&E> {
        if id < self.elements.len() {
            Some(self.elements[id].element_ref())
        } else {
            None
        }
    }

    fn child_mut(&mut self, id: usize) -> Option<&mut E> {
        self.shouldupdate = true;
        if id < self.elements.len() {
            Some(self.elements[id].element_mut())
        } else {
            None
        }
    }
}
