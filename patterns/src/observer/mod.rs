use std::{cell::Cell, rc::Weak};

pub enum Event {
    Mention { person: String },
    Comment { person: String, comment: String },
}

pub trait EventListener {
    fn update(&self, event: &Event);
}

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
}

impl EventListener for EmailAlerts {
    fn update(&self, event: &Event) {
        let count = self.count.get();
        self.count.set(count + 1);
        match event {
            Event::Mention { person } => {
                println!("{person} mentioned you!");
            }
            Event::Comment { person, .. } => {
                println!("{person} commented on your project.");
            }
        }
    }
}

#[derive(Default)]
pub struct Subject {
    listeners: Vec<Weak<dyn EventListener>>,
}

impl Subject {
    fn add_listener(&mut self, listener: Weak<dyn EventListener>) {
        self.listeners.push(listener);
    }

    fn remove_listener(&mut self, listener: Weak<dyn EventListener>) {
        self.listeners.retain(|x| !Weak::ptr_eq(x, &listener));
    }

    fn notify(&self, event: &Event) {
        for listener in self.listeners.iter() {
            if let Some(listener) = listener.upgrade() {
                listener.update(event);
            }
        }
    }
}

pub struct Editor {
    events: Subject,
    mentions: i32,
    comments: i32,
}

impl Editor {
    pub fn new() -> Editor {
        Editor {
            events: Subject::default(),
            mentions: 0,
            comments: 0,
        }
    }

    pub fn add_listener(&mut self, listener: Weak<dyn EventListener>) {
        self.events.add_listener(listener);
    }

    pub fn remove_listener(&mut self, listener: Weak<dyn EventListener>) {
        self.events.remove_listener(listener);
    }
    pub fn comments(&self) -> i32 {
        self.comments
    }

    pub fn mentions(&self) -> i32 {
        self.mentions
    }

    pub fn mention(&mut self, person: String) {
        // do stuff internally as an editor
        self.mentions += 1;

        // do event dispatching
        let event = Event::new_mention(person.clone());
        self.events.notify(&event);
    }

    pub fn comment(&mut self, person: String, comment: String) {
        // do stuff internally as an editor
        self.comments += 1;

        // do event dispatching
        let event = Event::new_comment(person.clone(), comment.clone());
        self.events.notify(&event);
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::*;

    #[test]
    fn basic_test() {
        // a publisher that emits the events
        let mut editor = Editor::new();

        // a subscriber that subscribes to the publisher
        let email_alerts = Rc::new(EmailAlerts::new());
        let email_alerts_listener = Rc::downgrade(&email_alerts);
        editor.add_listener(email_alerts_listener.clone());

        // update the state of the editor
        for _ in 0..5 {
            editor.comment(String::from("John"), String::from("Some comment"));
            editor.mention(String::from("Alice"));
        }

        editor.remove_listener(email_alerts_listener.clone());

        // confirm that the subscriber received the events
        let alerts_count = email_alerts.count();
        let editor_count = editor.comments() + editor.mentions();
        assert_eq!(alerts_count, editor_count);
    }
}
