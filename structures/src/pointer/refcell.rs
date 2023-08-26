use std::error::Error;
use std::fmt::{self, Debug, Display};
use std::marker::Unsize;
use std::mem;
use std::ops::{CoerceUnsized, Deref, DerefMut};
use std::ptr::NonNull;

use super::Cell;
use super::UnsafeCell;

type BorrowFlag = isize;
const EXCLUSIVE: BorrowFlag = -1;
const UNSHARED: BorrowFlag = 0;

pub struct BorrowError {}

impl Error for BorrowError {}

impl Debug for BorrowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut builder = f.debug_struct("BorrowError");
        builder.finish()
    }
}

#[inline]
const fn is_exclusive(borrow: BorrowFlag) -> bool {
    borrow == EXCLUSIVE
}

#[inline]
const fn is_shared(borrow: BorrowFlag) -> bool {
    borrow > 0
}

impl Display for BorrowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt("already mutably borrowed", f)
    }
}

pub struct BorrowMutError {}

impl Error for BorrowMutError {}

impl Debug for BorrowMutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut builder = f.debug_struct("BorrowMutError");
        builder.finish()
    }
}

impl Display for BorrowMutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt("already borrowed", f)
    }
}

pub struct RefCell<T>
where
    T: ?Sized,
{
    borrow: Cell<BorrowFlag>,
    value: UnsafeCell<T>,
}

impl<T> RefCell<T> {
    #[inline]
    pub fn new(value: T) -> Self {
        RefCell {
            value: UnsafeCell::new(value),
            borrow: Cell::new(UNSHARED),
        }
    }

    pub fn into_inner(self) -> T {
        self.value.into_inner()
    }

    #[inline]
    pub fn replace(&self, t: T) -> T {
        mem::replace(&mut *self.borrow_mut(), t)
    }

    #[inline]
    pub fn swap(&self, other: &Self) {
        mem::swap(&mut *self.borrow_mut(), &mut *other.borrow_mut())
    }
}

impl<T: ?Sized> RefCell<T> {
    #[inline]
    pub fn borrow(&self) -> Ref<'_, T> {
        self.try_borrow().unwrap()
    }

    #[inline]
    pub fn try_borrow(&self) -> Result<Ref<'_, T>, BorrowError> {
        if let Some(borrow) = BorrowRef::new(&self.borrow) {
            let value = unsafe { NonNull::new_unchecked(self.value.get()) };
            Ok(Ref { value, borrow })
        } else {
            Err(BorrowError {})
        }
    }

    #[inline]
    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        self.try_borrow_mut().unwrap()
    }

    #[inline]
    pub fn try_borrow_mut(&self) -> Result<RefMut<'_, T>, BorrowMutError> {
        if let Some(borrow) = BorrowRefMut::new(&self.borrow) {
            let value = unsafe { NonNull::new_unchecked(self.value.get()) };
            Ok(RefMut { value, borrow })
        } else {
            Err(BorrowMutError {})
        }
    }

    #[inline]
    pub fn as_ptr(&self) -> *mut T {
        self.value.get()
    }

    #[inline]
    pub fn get_mut(&mut self) -> &mut T {
        self.value.get_mut()
    }
}

impl<T: Default> Default for RefCell<T> {
    fn default() -> Self {
        RefCell::new(Default::default())
    }
}

impl<T> Debug for RefCell<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(f)
    }
}

impl<T: CoerceUnsized<U>, U> CoerceUnsized<RefCell<U>> for RefCell<T> {}

pub struct BorrowRef<'b> {
    borrow: &'b Cell<BorrowFlag>,
}

impl<'b> BorrowRef<'b> {
    #[inline]
    pub fn new(borrow: &'b Cell<BorrowFlag>) -> Option<BorrowRef<'b>> {
        let b = borrow.get().wrapping_add(1);
        if is_shared(b) {
            borrow.set(b);
            Some(BorrowRef { borrow })
        } else {
            None
        }
    }
}

impl Drop for BorrowRef<'_> {
    fn drop(&mut self) {
        let b = self.borrow.get();
        self.borrow.set(b - 1);
    }
}

pub struct Ref<'b, T: ?Sized + 'b> {
    value: NonNull<T>,
    borrow: BorrowRef<'b>,
}

impl<T: Display> Display for Ref<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe { self.value.as_ref() }.fmt(f)
    }
}

impl<T: Debug> Debug for Ref<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe { self.value.as_ref() }.fmt(f)
    }
}

impl<T: ?Sized> Deref for Ref<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { self.value.as_ref() }
    }
}

impl<'b, T: Unsize<U>, U: ?Sized> CoerceUnsized<Ref<'b, U>> for Ref<'b, T> {}

pub struct BorrowRefMut<'b> {
    borrow: &'b Cell<BorrowFlag>,
}

impl<'b> BorrowRefMut<'b> {
    #[inline]
    pub fn new(borrow: &'b Cell<BorrowFlag>) -> Option<BorrowRefMut<'b>> {
        if borrow.get() == UNSHARED {
            borrow.set(UNSHARED - 1);
            Some(BorrowRefMut { borrow })
        } else {
            None
        }
    }
}

impl Drop for BorrowRefMut<'_> {
    fn drop(&mut self) {
        self.borrow.set(UNSHARED);
    }
}

pub struct RefMut<'b, T: ?Sized> {
    value: NonNull<T>,
    borrow: BorrowRefMut<'b>,
}

impl<T: Display> Display for RefMut<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe { self.value.as_ref() }.fmt(f)
    }
}

impl<T: Debug> Debug for RefMut<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe { self.value.as_ref() }.fmt(f)
    }
}

impl<T: ?Sized> Deref for RefMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.value.as_ref() }
    }
}

impl<T: ?Sized> DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.value.as_mut() }
    }
}

impl<'b, T: Unsize<U>, U: ?Sized> CoerceUnsized<RefMut<'b, U>> for RefMut<'b, T> {}
