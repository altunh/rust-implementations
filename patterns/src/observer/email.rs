use crate::observer::events::{EditorEvent, ListenerUpdate};
use std::cell::Cell;

pub struct EmailAlerts {
    count: Cell<i32>,
}

impl EmailAlerts {
    pub fn new() -> EmailAlerts {
        EmailAlerts {
            count: Cell::new(0),
        }
    }

    pub fn count(&self) -> i32 {
        self.count.get()
    }

    pub fn reset(&mut self) {
        self.count.set(0);
    }
}

impl ListenerUpdate for EmailAlerts {
    fn update(&mut self, event: &EditorEvent) {
        let count = self.count.get();
        self.count.set(count + 1);
        match event {
            EditorEvent::Mention { person } => {
                eprintln!("{person} mentioned you!");
            }
            EditorEvent::Comment { person, .. } => {
                eprintln!("{person} commented on your project.");
            }
        }
    }
}
