use crate::drawable::Drawable;
use crate::gwin::window::Render;
use std::cmp::max;

#[derive(Clone, Copy)]
pub enum ChainType {
    Bottom,
    Right,
}

pub struct List<T: Drawable> {
    chain: ChainType,
    elements: Vec<T>,
}

impl<T: Drawable> List<T> {
    pub fn new(chain: ChainType) -> List<T> {
        List {
            chain,
            elements: Vec::new(),
        }
    }

    pub fn push(&mut self, e: T) {
        self.elements.push(e);
    }
}

impl<T: Drawable> Drawable for List<T> {
    fn size(&self) -> (usize, usize) {
        let mut s = (0, 0);
        for element in self.elements.iter() {
            let sz = element.size();
            match self.chain {
                ChainType::Right => {
                    s.0 += sz.0;
                    s.1 = max(s.1, sz.1);
                }
                ChainType::Bottom => {
                    s.0 = max(s.0, sz.0);
                    s.1 += sz.1;
                }
            }
        }
        s
    }

    fn draw(&self, r: &mut Render) {
        for element in self.elements.iter() {
            r.draw(element);
            let sz = element.size();
            match self.chain {
                ChainType::Right => {
                    r.moveby((sz.0, 0));
                }
                ChainType::Bottom => {
                    r.moveby((0, sz.1));
                }
            }
        }
    }
}
