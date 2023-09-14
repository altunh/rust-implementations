use crate::proxy::memo::{FnMemo, Memo};
use crate::proxy::node::NodeId;
use crate::proxy::runtime::{with_runtime, Runtime, RuntimeContext};
use std::any::Any;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

pub(crate) type FnUpdate<T> = impl FnOnce(&mut T);

pub(crate) type FnWatch<T> = impl Fn(&Option<T>);

pub struct Reference<T> {
    pub(crate) id: NodeId,
    pub(crate) ty: PhantomData<T>,
}

impl<T> Reference<T> {
    pub(crate) fn new(id: NodeId) -> Self {
        Reference {
            id,
            ty: PhantomData,
        }
    }

    pub fn update(&self, f: FnUpdate<T>) {}

    pub fn watch(&self, f: FnWatch<T>) {}

    pub fn memo(&self, f: FnMemo<T>) -> Memo<T> {
        let memo = RuntimeContext.create_memo(f);
        memo
    }

    pub fn as_ref(&self) -> &T {
        self.val.borrow().deref()
    }

    pub(crate) fn try_with<O>(&self, f: impl FnOnce(&T) -> O) -> Option<O> {
        with_runtime(|runtime| self.id.try_with(runtime, f))
    }
}

impl<T: Copy> Reference<T> {
    pub fn get(&self) -> T {
        self.try_with()
    }
}

pub fn reference<T>(value: T) -> Reference<T> {
    RuntimeContext.create_ref(value)
}
