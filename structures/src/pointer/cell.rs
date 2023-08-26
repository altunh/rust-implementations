use super::UnsafeCell;

use std::cmp::Ordering;
use std::fmt::{self, Debug, Display};
use std::ptr;

/*
 * TODO:
 * - [ ] impl CoerceUnsized for Cell
 * - [ ] impl DispatchFromDyn for Cell
 * - [ ] impl Cell<[T]>
 * - [ ] impl Cell<[T; N]>
 */

#[repr(transparent)]
pub struct Cell<T: ?Sized> {
    value: UnsafeCell<T>,
}

impl<T> Cell<T> {
    pub fn new(value: T) -> Cell<T> {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    #[inline]
    pub fn set(&self, value: T) {
        unsafe { *self.value.get() = value }
    }

    #[inline]
    pub fn swap(&self, other: &Cell<T>) {
        if !ptr::eq(self, other) {
            unsafe {
                ptr::swap(self.value.get(), other.value.get());
            }
        }
    }

    #[inline]
    pub fn replace(&self, value: T) -> T {
        unsafe { self.value.get().replace(value) }
    }

    pub fn into_inner(self) -> T {
        self.value.into_inner()
    }
}

impl<T: Copy> Cell<T> {
    pub fn get(&self) -> T {
        unsafe { *self.value.get() }
    }

    pub fn update<F>(&self, f: F) -> T
    where
        F: FnOnce(T) -> T,
    {
        let old = self.get();
        let new = f(old);
        self.set(new);
        new
    }
}

impl<T: ?Sized> Cell<T> {
    #[inline]
    pub fn from_mut(t: &mut T) -> &Cell<T> {
        unsafe { &*(t as *mut T as *const Cell<T>) }
    }

    #[inline]
    pub const fn as_ptr(&self) -> *mut T {
        self.value.get()
    }

    #[inline]
    pub fn get_mut(&mut self) -> &mut T {
        self.value.get_mut()
    }
}

impl<T: Default> Cell<T> {
    pub fn take(&self) -> T {
        self.replace(Default::default())
    }
}

impl<T: Copy> Clone for Cell<T> {
    #[inline]
    fn clone(&self) -> Cell<T> {
        Cell::new(self.get())
    }
}

impl<T: Default> Default for Cell<T> {
    #[inline]
    fn default() -> Cell<T> {
        Self::new(Default::default())
    }
}

impl<T> From<T> for Cell<T> {
    fn from(t: T) -> Cell<T> {
        Cell::new(t)
    }
}

impl<T: Eq + Copy> Eq for Cell<T> {}

impl<T: PartialEq + Copy> PartialEq for Cell<T> {
    #[inline]
    fn eq(&self, other: &Cell<T>) -> bool {
        self.get() == other.get()
    }
}

impl<T: Ord + Copy> Ord for Cell<T> {
    #[inline]
    fn cmp(&self, other: &Cell<T>) -> Ordering {
        self.get().cmp(&other.get())
    }
}

impl<T: PartialOrd + Copy> PartialOrd for Cell<T> {
    #[inline]
    fn partial_cmp(&self, other: &Cell<T>) -> Option<Ordering> {
        self.get().partial_cmp(&other.get())
    }

    #[inline]
    fn lt(&self, other: &Cell<T>) -> bool {
        self.get() < other.get()
    }

    #[inline]
    fn le(&self, other: &Cell<T>) -> bool {
        self.get() <= other.get()
    }

    #[inline]
    fn gt(&self, other: &Cell<T>) -> bool {
        self.get() > other.get()
    }

    #[inline]
    fn ge(&self, other: &Cell<T>) -> bool {
        self.get() >= other.get()
    }
}

impl<T: Debug + Copy> Debug for Cell<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cell {{ value: {:?} }}", self.get())
    }
}

impl<T: Display + Copy> Display for Cell<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get())
    }
}
