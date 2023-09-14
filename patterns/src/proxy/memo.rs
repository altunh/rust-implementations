use crate::proxy::effect::AnyComputation;
use crate::proxy::node::NodeId;
use std::any::Any;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

pub(crate) type FnMemo<T> = impl Fn(&T) -> T;

pub struct Memo<T> {
    id: NodeId,
    ty: PhantomData<T>,
}

impl<T> Memo<T> {
    pub(crate) fn new(id: NodeId) -> Self {
        Self {
            id,
            ty: PhantomData,
        }
    }

    pub fn as_ref(&self) -> &T {
        self.val.borrow().deref()
    }
}

impl<T: Copy> Memo<T> {
    pub fn get(&self) -> T {
        self.val.borrow().to_owned()
    }
}

pub struct MemoState<T> {
    f: FnMemo<T>,
    ty: PhantomData<T>,
}

impl<T> MemoState<T> {
    pub(crate) fn new(f: FnMemo<T>) -> Self {
        Self { f, ty: PhantomData }
    }
}

impl<T> AnyComputation for MemoState<T> {
    fn run(&self, value: Rc<RefCell<dyn Any>>) -> bool {
        let (new_value, is_different) = {
            let curr_value = value.borrow().downcast_ref::<Option<&T>>();
            let new_value = (self.f)(curr_value.as_ref());
            let is_different = curr_value.as_ref() != Some(&new_value);
            (new_value, is_different)
        };
        if is_different {
            let mut curr_value = value.borrow_mut().downcast_mut().unwrap();
            *curr_value = Some(new_value);
        }
        is_different
    }
}
