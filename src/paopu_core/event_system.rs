use std::cell::RefCell;
use std::rc::Rc;

use crate::core::events::PaopuEvent;

pub trait Observer {
    fn on_notify(&mut self, event: &PaopuEvent);
}

pub struct EventSystem {
    observers: Vec<Rc<RefCell<dyn Observer>>>,
    pub events: Vec<PaopuEvent>,
}

impl EventSystem {
    pub fn new() -> EventSystem {
        EventSystem { 
            observers: vec![],
            events: vec![],
        }
    }

    pub fn notify(&self, event: PaopuEvent) {
        for observer_wrapper in self.observers.clone() {
            let mut observer = observer_wrapper.borrow_mut();
            observer.on_notify(&event);
        }
    }

    pub fn add_observer(&mut self, observer: Rc<RefCell<dyn Observer>>) {
        self.observers.push(observer);
    }
}
