use crate::drawable::Drawable;
use crate::gwin::window::Window;
use crossterm::event::*;
use std::mem;
use std::sync::{
    mpsc,
    mpsc::{Receiver, Sender},
};

pub mod event {
    pub use crossterm::event::*;
}

pub type FnElement<S> = fn(&S) -> Box<dyn Drawable>;
pub type EventHandle<S> = fn(Event, Sender<Mutator<S>>);
pub type Mutator<S> = Box<dyn Fn(S) -> S + Send>;

pub fn mutate<T>(value: &mut T, mutator: Mutator<T>) {
    unsafe {
        let val = mem::replace(value, mem::zeroed());
        let _ = mem::replace(value, mutator(val));
    }
}

pub fn send_tx<S, F: Fn(S) -> S + Send + 'static>(tx: &Sender<Mutator<S>>, mutator: F) {
    tx.send(Box::new(mutator)).expect("Failed to use tx");
}

pub struct Root<S> {
    // state
    state: S,
    shouldupdate: bool,
    rx: Receiver<Mutator<S>>,
    tx: Sender<Mutator<S>>,
    // etc
    element: FnElement<S>,
    event_handle: EventHandle<S>,
    window: Window,
}

impl<S> Root<S> {
    pub fn new(state: S, element: FnElement<S>, event_handle: EventHandle<S>) -> Self {
        let (tx, rx) = mpsc::channel();
        Root {
            event_handle,
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

    pub fn events(&mut self) {
        while poll(std::time::Duration::from_secs(0)).expect("Failed to poll crossterm event") {
            (self.event_handle)(read().expect("Failed to read crossterm event"), self.tx());
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

            self.events();

            self.receive();

            std::thread::sleep(period);
        }
    }
}
