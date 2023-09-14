use crate::proxy::effect::AnyComputation;
use crate::proxy::runtime::Runtime;
use slotmap::new_key_type;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

new_key_type! {
    pub struct NodeId;
}

pub(crate) enum ReactiveError {
    Disposed,
    BadCast,
}

impl NodeId {
    pub fn try_with<T, U>(
        &self,
        runtime: &Runtime,
        f: impl FnOnce(&T) -> U,
    ) -> Result<U, ReactiveError> {
    }
}

pub(crate) enum ReactiveNodeType {
    Reference,
    Memo { f: Rc<dyn AnyComputation> },
    Computed { f: Rc<dyn AnyComputation> },
    Effect { f: Rc<dyn AnyComputation> },
}

pub(crate) struct ReactiveNode {
    pub value: Option<Rc<RefCell<dyn Any>>>,
    pub node_type: ReactiveNodeType,
}
