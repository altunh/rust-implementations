use crate::observer::events::EditorEvent;
use crate::observer::subject::Subject;

pub struct Editor {
    listeners: Subject,
    mentions: i32,
    comments: i32,
}

impl Editor {
    pub fn new() -> Editor {
        Editor {
            listeners: Subject::default(),
            mentions: 0,
            comments: 0,
        }
    }

    pub fn listeners(&mut self) -> &mut Subject {
        &mut self.listeners
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
        let event = EditorEvent::Mention {
            person: person.clone(),
        };
        self.listeners.notify(&event);
    }

    pub fn comment(&mut self, person: String, comment: String) {
        // do stuff internally as an editor
        self.comments += 1;

        // do event dispatching
        let event = EditorEvent::Comment {
            person: person.clone(),
            comment: comment.clone(),
        };
        self.listeners.notify(&event);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::observer::email::EmailAlerts;
    use crate::observer::events::ListenerCell;

    #[test]
    fn basic_test() {
        // a publisher that emits the events
        let mut editor = Editor::new();

        // a subscriber that subscribes to the publisher
        let alerts = EmailAlerts::new();
        let alerts = ListenerCell::from(alerts);
        editor.listeners().add_listener(&alerts);

        // update the state of the editor
        for _ in 0..5 {
            editor.comment(String::from("John"), String::from("Some comment"));
            editor.mention(String::from("Alice"));
        }

        // confirm that the subscriber received the events
        let editor_count = editor.comments() + editor.mentions();
        assert_eq!(alerts.as_ref().count(), editor_count);

        // remove the listener
        editor.listeners().remove_listener(&alerts);

        for _ in 0..10 {
            editor.mention(String::from("John"))
        }

        // confirm that the subscriber didn't receive the events after removal
        assert_eq!(alerts.as_ref().count(), editor_count);
        assert_ne!(
            alerts.as_ref().count(),
            editor.comments() + editor.mentions()
        );
    }
}
