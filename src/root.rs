use crate::drawable::Drawable;
use crate::gwin::window::Window;

pub type FnElement<S, D> = fn(&S) -> D;

pub struct Root<S, D: Drawable> {
    state: S,
    element: FnElement<S, D>,
    window: Window,
}

impl<S, D: Drawable> Root<S, D> {
    pub fn new(state: S, element: FnElement<S, D>) -> Self {
        Root {
            state,
            element,
            window: Window::new(),
        }
    }

    pub fn render(&mut self) {
        self.window.push_drawable(&(self.element)(&self.state));
    }
}
