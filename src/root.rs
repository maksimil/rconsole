use crate::drawable::Drawable;
use crate::gwin::window::Window;
use std::mem;
use std::sync::{
    mpsc,
    mpsc::{Receiver, Sender},
};

pub type FnElement<S> = fn(&S) -> Box<dyn Drawable>;
pub type Mutator<S> = fn(S) -> S;

pub fn mutate<T>(value: &mut T, mutator: Mutator<T>) {
    unsafe {
        let val = mem::replace(value, mem::zeroed());
        let _ = mem::replace(value, mutator(val));
    }
}

pub struct Root<S> {
    // state
    state: S,
    shouldupdate: bool,
    rx: Receiver<Mutator<S>>,
    tx: Sender<Mutator<S>>,
    // etc
    element: FnElement<S>,
    window: Window,
}

impl<S> Root<S> {
    pub fn new(state: S, element: FnElement<S>) -> Self {
        let (tx, rx) = mpsc::channel();
        Root {
            rx,
            tx,
            state,
            shouldupdate: true,
            element,
            window: Window::new(),
        }
    }

    pub fn shouldupdate(&self) -> bool {
        self.shouldupdate
    }

    pub fn receive(&mut self) {
        for mutator in self.rx.try_recv() {
            self.shouldupdate = true;
            mutate(&mut self.state, mutator);
        }
    }

    pub fn tx(&self) -> Sender<Mutator<S>> {
        self.tx.clone()
    }

    pub fn render(&mut self) {
        self.window.push_drawable(&(self.element)(&self.state));
    }

    pub fn run_loop(&mut self, period: std::time::Duration) {
        loop {
            if self.shouldupdate() {
                self.render();
            }

            self.receive();

            std::thread::sleep(period);
        }
    }
}
