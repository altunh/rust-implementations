use std::cell::UnsafeCell;

use super::cell::Cell;

type BorrowFlag = isize;

const UNSHARED: BorrowFlag = 0;

#[inline]
const fn is_exclusive(borrow: BorrowFlag) -> bool {
    borrow < 0
}

#[inline]
const fn is_shared(borrow: BorrowFlag) -> bool {
    borrow > 0
}

pub struct RefCell<T> {
    value: UnsafeCell<T>,
    borrow: Cell<BorrowFlag>,
}

impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        RefCell {
            value: UnsafeCell::new(value),
            borrow: Cell::new(0)
        }
    }
}