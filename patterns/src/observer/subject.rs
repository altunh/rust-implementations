use crate::observer::events::{EditorEvent, ListenerCell, ListenerRef, ListenerUpdate};

#[derive(Default)]
pub struct Subject {
    listeners: Vec<ListenerRef>,
}

impl Subject {
    pub fn new() -> Self {
        Subject::default()
    }

    pub fn add_listener<T: ListenerUpdate + 'static>(&mut self, listener: &ListenerCell<T>) {
        let listener = listener.get_ref();
        self.listeners.push(listener);
    }

    pub fn remove_listener<T: ListenerUpdate + 'static>(&mut self, listener: &ListenerCell<T>) {
        let listener = listener.get_ref();
        self.listeners.retain(|x| x != &listener);
    }

    pub fn try_notify(&mut self, event: &EditorEvent) {
        for listener in self.listeners.iter_mut() {
            let _ = listener.try_update(event);
        }
    }

    pub fn notify(&mut self, event: &EditorEvent) {
        for listener in self.listeners.iter_mut() {
            listener.update(event);
        }
    }

    pub fn listener_count(&self) -> usize {
        self.listeners
            .iter()
            .filter(|listener| listener.is_valid())
            .count()
    }
}
