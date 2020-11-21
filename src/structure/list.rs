use crate::render::{render_struct, NextPos, NextPos::*};
use crate::structure::Structure;
use std::cmp::max;

pub struct ListStructure {
    offset: (u16, u16),
    chained: NextPos,
    children: Vec<Box<dyn Structure>>,
}

impl ListStructure {
    pub fn new(chained: NextPos, offset: (u16, u16)) -> ListStructure {
        ListStructure {
            offset,
            chained,
            children: Vec::new(),
        }
    }

    pub fn push<T: Structure + 'static>(&mut self, e: T) {
        self.children.push(Box::new(e));
    }
}

impl Structure for ListStructure {
    fn offset(&self) -> (u16, u16) {
        self.offset
    }

    fn size(&self) -> (u16, u16) {
        let mut s = (0, 0);
        match self.chained {
            Right => {
                for child in self.children.iter() {
                    let (cw, ch) = child.size();
                    let (ow, oh) = child.offset();
                    s.0 += cw + ow;
                    s.1 = max(s.1, ch + oh);
                }
            }
            Bottom => {
                for child in self.children.iter() {
                    let (cw, ch) = child.size();
                    let (ow, oh) = child.offset();
                    s.0 = max(s.0, cw + ow);
                    s.1 += ch + oh;
                }
            }
        }
        s
    }

    fn render(&self) {
        for child in self.children.iter() {
            render_struct(child, self.chained);
        }
    }
}
