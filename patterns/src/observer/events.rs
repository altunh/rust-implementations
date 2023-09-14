use std::cell::{Ref, RefCell, RefMut};
use std::rc::{Rc, Weak};

/// Events that can be emitted represented using an enum
pub enum EditorEvent {
    Mention { person: String },
    Comment { person: String, comment: String },
}

/// Interface for a listener
pub trait ListenerUpdate {
    fn update(&mut self, event: &EditorEvent);
}

/// `Listener` wraps the updater `EventListener` and gives out references.
pub struct ListenerCell<T> {
    rc: Rc<RefCell<T>>,
}

impl<T> ListenerCell<T>
where
    T: ListenerUpdate + 'static,
{
    pub fn from(listener: T) -> Self {
        ListenerCell {
            rc: Rc::new(RefCell::new(listener)),
        }
    }

    /// This is used by `Subject` to obtain the listener.
    pub fn get_ref(&self) -> ListenerRef {
        let rc: Rc<RefCell<dyn ListenerUpdate>> = self.rc.clone();
        ListenerRef {
            weak: Rc::downgrade(&rc),
        }
    }

    pub fn as_ref(&self) -> Ref<'_, T> {
        self.rc.borrow()
    }

    pub fn as_mut(&mut self) -> RefMut<'_, T> {
        self.rc.borrow_mut()
    }
}

impl<T> Clone for ListenerCell<T> {
    fn clone(&self) -> Self {
        ListenerCell {
            rc: self.rc.clone(),
        }
    }
}

pub struct ListenerRef {
    weak: Weak<RefCell<dyn ListenerUpdate>>,
}

pub enum ListenerError {
    Invalid,
}

impl ListenerRef {
    #[inline]
    pub fn is_valid(&self) -> bool {
        self.weak.upgrade().is_some()
    }

    pub fn try_update(&mut self, event: &EditorEvent) -> Result<(), ListenerError> {
        if let Some(rc) = self.weak.upgrade() {
            rc.borrow_mut().update(event);
            Ok(())
        } else {
            Err(ListenerError::Invalid)
        }
    }

    pub fn update(&mut self, event: &EditorEvent) {
        match self.try_update(event) {
            Ok(_) => (),
            Err(ListenerError::Invalid) => panic!("Listener doesn't exist anymore"),
        }
    }
}

impl Eq for ListenerRef {}

impl PartialEq for ListenerRef {
    fn eq(&self, other: &Self) -> bool {
        self.weak.ptr_eq(&other.weak)
    }
}

#[cfg(test)]
mod tests {
    use crate::observer::email::EmailAlerts;
    use crate::observer::subject::Subject;

    use super::*;

    #[test]
    fn listener_drop() {
        let listener = EmailAlerts::new();
        let mut subject = Subject::new();
        {
            let listener = ListenerCell::from(listener);
            assert_eq!(Rc::weak_count(&listener.rc), 0);
            assert_eq!(Rc::strong_count(&listener.rc), 1);
            subject.add_listener(&listener);
            subject.add_listener(&listener);
            assert_eq!(subject.listener_count(), 2);
            assert_eq!(Rc::weak_count(&listener.rc), 2);
            assert_eq!(Rc::strong_count(&listener.rc), 1);
        }
        assert_eq!(subject.listener_count(), 0);
    }
}
