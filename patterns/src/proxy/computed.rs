use crate::proxy::node::NodeId;
use crate::proxy::runtime::RuntimeContext;
use std::marker::PhantomData;

pub struct Computed<T> {
    id: NodeId,
    ty: PhantomData<T>,
}

impl<T> Computed<T> {
    pub(crate) fn new(id: NodeId) -> Self {
        Self {
            id,
            ty: PhantomData,
        }
    }

    pub fn get() -> T {}
}

pub fn computed<T>(f: impl Fn() -> T) -> Computed<T> {
    RuntimeContext.create_computed(f)
}
