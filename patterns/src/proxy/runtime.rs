use crate::proxy::memo::{FnMemo, Memo, MemoState};
use crate::proxy::node::{NodeId, ReactiveNode, ReactiveNodeType};
use crate::proxy::reference::Reference;
use slotmap::{SecondaryMap, SlotMap};
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

thread_local! {
    pub(crate) static RUNTIME: Runtime = Runtime::new();
}

pub struct Runtime {
    nodes: RefCell<SlotMap<NodeId, ReactiveNode>>,
    node_subscribers: RefCell<SecondaryMap<NodeId, RefCell<HashSet<NodeId>>>>,
}

impl Runtime {
    pub(crate) fn new() -> Self {
        Self {
            nodes: RefCell::new(SlotMap::default()),
            node_subscribers: RefCell::new(SecondaryMap::default()),
        }
    }
}

pub(crate) fn with_runtime<T, F>(f: F) -> T
where
    F: FnOnce(&Runtime) -> T,
{
    RUNTIME.with(|runtime| f(runtime))
}

pub(crate) struct RuntimeContext;

impl RuntimeContext {
    pub(crate) fn create_ref<T>(self, value: T) -> Reference<T> {
        let id = with_runtime(|runtime| {
            runtime.nodes.borrow_mut().insert(ReactiveNode {
                value: Some(value),
                node_type: ReactiveNodeType::Ref,
            })
        });
        Reference::new(id)
    }

    pub(crate) fn create_memo<T>(self, f: FnMemo<T>) -> Memo<T> {
        let state = Rc::new(MemoState::new(f));
        let id = with_runtime(|runtime| {
            runtime.nodes.borrow_mut().insert(ReactiveNode {
                value: Some(Rc::new(RefCell::new(None::<T>))),
                node_type: ReactiveNodeType::Memo { f: state },
            })
        });
        Memo::new(id)
    }
}
