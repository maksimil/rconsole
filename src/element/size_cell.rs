use crate::element::Element;
use crate::render::clear_zone;

pub struct SizeCell<E: Element> {
    element: E,
    fsize: (u16, u16),
}

impl<E: Element> SizeCell<E> {
    pub fn new(element: E) -> SizeCell<E> {
        let fsize = element.size();
        SizeCell { element, fsize }
    }

    pub fn element_ref(&self) -> &E {
        &self.element
    }

    pub fn element_mut(&mut self) -> &mut E {
        &mut self.element
    }
}

impl<E: Element> Element for SizeCell<E> {
    fn size(&self) -> (u16, u16) {
        self.element.size()
    }

    fn render_inner(&mut self) {
        clear_zone(self.fsize);
        self.fsize = self.size();
        self.element.render();
    }

    fn shouldupdate(&self) -> bool {
        self.element.shouldupdate()
    }
}
