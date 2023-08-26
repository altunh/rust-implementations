use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{self, Debug, Display};
use std::marker::{PhantomData, Unsize};
use std::mem;
use std::ops::{CoerceUnsized, Deref, DerefMut};
use std::ptr::NonNull;

use super::Cell;
use super::UnsafeCell;

type BorrowFlag = isize;
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
    pub fn replace_with<F: FnOnce(&mut T) -> T>(&self, f: F) -> T {
        let mut_borrow = &mut *self.borrow_mut();
        let replacement = f(mut_borrow);
        mem::replace(mut_borrow, replacement)
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
            Ok(RefMut {
                value,
                borrow,
                marker: PhantomData,
            })
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

impl<T: Default> RefCell<T> {
    pub fn take(&self) -> T {
        self.replace(Default::default())
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

impl<T: Clone> Clone for RefCell<T> {
    fn clone(&self) -> Self {
        RefCell::new(self.borrow().clone())
    }

    fn clone_from(&mut self, source: &Self) {
        self.get_mut().clone_from(&source.borrow())
    }
}

impl<T> From<T> for RefCell<T> {
    fn from(value: T) -> Self {
        RefCell::new(value)
    }
}

impl<T: Eq + ?Sized> Eq for RefCell<T> {}

impl<T: PartialEq + ?Sized> PartialEq for RefCell<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        *self.borrow() == *other.borrow()
    }
}

impl<T: Ord + ?Sized> Ord for RefCell<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.borrow().cmp(&*other.borrow())
    }
}

impl<T: PartialOrd + ?Sized> PartialOrd for RefCell<T> {
    #[inline]
    fn partial_cmp(&self, other: &RefCell<T>) -> Option<Ordering> {
        self.borrow().partial_cmp(&*other.borrow())
    }

    #[inline]
    fn lt(&self, other: &RefCell<T>) -> bool {
        *self.borrow() < *other.borrow()
    }

    #[inline]
    fn le(&self, other: &RefCell<T>) -> bool {
        *self.borrow() <= *other.borrow()
    }

    #[inline]
    fn gt(&self, other: &RefCell<T>) -> bool {
        *self.borrow() > *other.borrow()
    }

    #[inline]
    fn ge(&self, other: &RefCell<T>) -> bool {
        *self.borrow() >= *other.borrow()
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

impl Clone for BorrowRef<'_> {
    fn clone(&self) -> Self {
        let borrow = self.borrow.get();
        self.borrow.set(borrow + 1);
        BorrowRef {
            borrow: self.borrow,
        }
    }
}

impl Drop for BorrowRef<'_> {
    fn drop(&mut self) {
        let borrow = self.borrow.get();
        self.borrow.set(borrow - 1);
    }
}

pub struct Ref<'b, T: ?Sized + 'b> {
    value: NonNull<T>,
    borrow: BorrowRef<'b>,
}

impl<'b, T: ?Sized> Ref<'b, T> {
    pub fn clone(orig: &Ref<'b, T>) -> Ref<'b, T> {
        Ref {
            value: orig.value,
            borrow: orig.borrow.clone(),
        }
    }

    pub fn map<U, F>(orig: Ref<'b, T>, f: F) -> Ref<'b, U>
    where
        F: FnOnce(&T) -> &U,
        U: ?Sized,
    {
        Ref {
            value: NonNull::from(f(&*orig)),
            borrow: orig.borrow,
        }
    }

    pub fn filter_map<U, F>(orig: Ref<'b, T>, f: F) -> Result<Ref<'b, U>, Self>
    where
        F: FnOnce(&T) -> Option<&U>,
        U: ?Sized,
    {
        if let Some(value) = f(&*orig) {
            Ok(Ref {
                value: NonNull::from(value),
                borrow: orig.borrow,
            })
        } else {
            Err(orig)
        }
    }
}

impl<T: Display> Display for Ref<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (**self).fmt(f)
    }
}

impl<T: Debug> Debug for Ref<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (**self).fmt(f)
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

pub struct RefMut<'b, T: ?Sized + 'b> {
    value: NonNull<T>,
    borrow: BorrowRefMut<'b>,
    marker: PhantomData<&'b mut T>,
}

impl<'b, T: ?Sized> RefMut<'b, T> {
    pub fn map<U, F>(mut orig: RefMut<'b, T>, f: F) -> RefMut<'b, U>
    where
        F: FnOnce(&mut T) -> &mut U,
        U: ?Sized,
    {
        RefMut {
            value: NonNull::from(f(&mut *orig)),
            borrow: orig.borrow,
            marker: PhantomData,
        }
    }

    pub fn filter_map<U, F>(mut orig: RefMut<'b, T>, f: F) -> Result<RefMut<'b, U>, RefMut<'b, T>>
    where
        F: FnOnce(&mut T) -> Option<&mut U>,
        U: ?Sized,
    {
        if let Some(value) = f(&mut *orig) {
            Ok(RefMut {
                value: NonNull::from(value),
                borrow: orig.borrow,
                marker: PhantomData,
            })
        } else {
            Err(orig)
        }
    }
}

impl<T: Display> Display for RefMut<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (**self).fmt(f)
    }
}

impl<T: Debug> Debug for RefMut<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (**self).fmt(f)
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
