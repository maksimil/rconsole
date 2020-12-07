use crate::drawable::Drawable;
use crate::gwin::window::Render;
use std::cmp::max;

#[derive(Clone, Copy)]
pub enum ChainType {
    Bottom,
    Right,
}

pub struct List {
    chain: ChainType,
    elements: Vec<Box<dyn Drawable>>,
}

impl List {
    pub fn new(elements: Vec<Box<dyn Drawable>>, chain: ChainType) -> List {
        List { chain, elements }
    }

    pub fn push(&mut self, e: Box<dyn Drawable>) {
        self.elements.push(e);
    }
}

impl Drawable for List {
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
            let cp = r.cpos();
            r.draw(element);
            let sz = element.size();
            r.moveto(cp);
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

pub fn list(elements: Vec<Box<dyn Drawable>>, chain: ChainType) -> Box<dyn Drawable> {
    Box::new(List::new(elements, chain))
}
