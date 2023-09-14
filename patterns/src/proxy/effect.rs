use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

pub(crate) trait AnyComputation {
    fn run(&self, value: Rc<RefCell<dyn Any>>) -> bool;
}
