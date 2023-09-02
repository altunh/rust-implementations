use std::fmt::Debug;
use std::{marker::PhantomData, ptr::NonNull};
use std::{mem, ptr, slice};

pub struct IntoIter<T> {
    pub(super) buf: NonNull<T>,
    pub(super) phantom: PhantomData<T>,
    pub(super) cap: usize,
    pub(super) ptr: *const T,
    pub(super) end: *const T,
}

impl<T> IntoIter<T> {
    pub fn as_raw_mut_slice(&mut self) -> *mut [T] {
        ptr::slice_from_raw_parts_mut(self.ptr as *mut T, self.len())
    }

    pub fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.ptr, self.len()) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { &mut *self.as_raw_mut_slice() }
    }
}

impl<T> AsRef<[T]> for IntoIter<T> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        if self.ptr == self.end {
            None
        } else if is_zst::<T>() {
            self.end = self.end.wrapping_offset((1 as isize).wrapping_neg());
            Some(unsafe { mem::zeroed() })
        } else {
            let old = self.ptr;
            self.ptr = unsafe { self.ptr.add(1) };
            Some(unsafe { ptr::read(old) })
        }
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }
}

impl<T: Debug> Debug for IntoIter<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("IntoIter").field(&self.as_slice()).finish()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.ptr == self.end {
            None
        } else if is_zst::<T>() {
            self.end = self.end.wrapping_offset((1 as isize).wrapping_neg());
            Some(unsafe { mem::zeroed() })
        } else {
            self.end = unsafe { self.ptr.sub(1) };
            Some(unsafe { ptr::read(self.end) })
        }
    }
}

impl<T> Default for IntoIter<T> {
    fn default() -> Self {
        super::Vec::new().into_iter()
    }
}

impl<T: Clone> Clone for IntoIter<T> {
    fn clone(&self) -> Self {
        todo!()
    }
}

#[inline(always)]
const fn is_zst<T>() -> bool {
    mem::size_of::<T>() == 0
}
