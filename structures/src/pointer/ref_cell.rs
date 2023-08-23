use std::cell::UnsafeCell;

use super::cell::Cell;

type BorrowFlag = isize;

pub struct RefCell<T> {
    value: UnsafeCell<T>,
    borrow: Cell<BorrowFlag>,
}