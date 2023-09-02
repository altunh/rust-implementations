use std::marker::PhantomData;
use std::mem::{self, ManuallyDrop};
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::ptr::NonNull;
use std::slice::SliceIndex;
use std::{cmp, ptr, slice};

use super::iter::IntoIter;
use super::rawvec::{RawVec, TryReserveError};

pub struct Vec<T> {
    buf: RawVec<T>,
    len: usize,
}

impl<T> Vec<T> {
    #[inline]
    pub const fn new() -> Self {
        Self {
            buf: RawVec::NEW,
            len: 0,
        }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buf: RawVec::with_capacity(capacity),
            len: 0,
        }
    }

    #[inline]
    pub fn with_capacity_zeroed(capacity: usize) -> Self {
        Self {
            buf: RawVec::with_capacity_zeroed(capacity),
            len: 0,
        }
    }

    #[inline]
    pub fn from_raw_parts(buf: *mut T, len: usize, capacity: usize) -> Vec<T> {
        Vec {
            buf: RawVec::from_raw_parts(buf, capacity),
            len,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.buf.capacity()
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.buf.ptr()
    }

    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self.buf.ptr()
    }

    #[inline]
    pub(crate) unsafe fn set_len(&mut self, len: usize) {
        assert!(len <= self.capacity());
        self.len = len;
    }

    pub fn push(&mut self, value: T) {
        // if length reached capacity, request an additional space of 1
        if self.len == self.capacity() {
            self.buf.reserve_for_push(self.len);
        }

        // p.add(capacity) is the end of the last byte of allocated space
        // you can write an element T, at most, at location p.add(capacity - 1)
        // SAFETY: offset is valid, since len < capacity, so len <= (capacity - 1) < capacity
        unsafe {
            let end = self.as_mut_ptr().add(self.len);
            ptr::write(end, value);
            self.len += 1;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        // p.add(capacity) is the end of the last byte of allocated space
        // you can read an element T, at most, at location p.add(capacity - 1)
        // SAFETY: offset is valid, since len <= capacity, so (len - 1) <= (capacity - 1) < capacity
        self.len -= 1;
        unsafe {
            let end = self.as_ptr().add(self.len);
            Some(ptr::read(end))
        }
    }

    pub fn insert(&mut self, index: usize, element: T) {
        // check bounds
        let len = self.len();
        assert!(index <= len, "index out of bounds");

        // reserve space for the new element
        if len == self.capacity() {
            self.buf.reserve(len, 1);
        }

        // p.add(capacity) is the end of the last byte of allocated space
        // you can write an element T, at most, at location p.add(capacity - 1)
        // SAFETY: offset is valid, since index <= len <= (capacity - 1) < capacity
        unsafe {
            {
                let p = self.as_mut_ptr().add(index);
                // here, index < len, so (index + 1) <= len <= (capacity - 1) < capacity
                if index < len {
                    ptr::copy(p, p.add(1), len - index);
                }
                ptr::write(p, element);
            }
            self.set_len(len + 1);
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
        // check bounds
        let len = self.len();
        assert!(index < len, "index out of bounds");

        // p.add(capacity) is the end of the last byte of allocated space
        // you can write an element T, at most, at location p.add(capacity - 1)
        // SAFETY: offset is valid, since index < len <= (capacity - 1) < capacity
        unsafe {
            let value: T;
            {
                let p = self.as_mut_ptr().add(index);
                value = ptr::read(p);
                ptr::copy(p.add(1), p, len - index - 1);
            }
            self.set_len(len - 1);
            value
        }
    }

    pub fn swap_remove(&mut self, index: usize) -> T {
        // check bounds
        let len = self.len();
        assert!(index < len, "index out of bounds");

        unsafe {
            let value = ptr::read(self.as_ptr().add(index));
            let base_ptr = self.as_mut_ptr();
            ptr::copy(base_ptr.add(len - 1), base_ptr.add(index), 1);
            self.set_len(len - 1);
            value
        }
    }

    #[inline]
    pub fn into_raw_parts(self) -> (*mut T, usize, usize) {
        let mut m = ManuallyDrop::new(self);
        (m.as_mut_ptr(), m.len(), m.capacity())
    }

    pub fn reserve(&mut self, additional: usize) {
        self.buf.reserve(self.len, additional);
    }

    pub fn reserve_exact(&mut self, additional: usize) {
        self.buf.reserve_exact(self.len, additional);
    }

    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.buf.try_reserve(self.len, additional)
    }

    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.buf.try_reserve_exact(self.len, additional)
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        self
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self
    }

    #[inline]
    pub fn clear(&mut self) {
        let elems: *mut [T] = self.as_mut_slice();
        unsafe {
            self.len = 0;
            ptr::drop_in_place(elems);
        }
    }

    pub fn extend<I: Iterator<Item = T>>(&mut self, mut iterator: I) {
        // Main loop after optimizations taken from Vec::extend_desugared.
        // Writes the rest of the elements from the iterator to the vector.
        while let Some(element) = iterator.next() {
            let len = self.len();
            if len == self.capacity() {
                let (lower, _) = iterator.size_hint();
                self.reserve(lower.saturating_add(1));
            }
            unsafe {
                ptr::write(self.as_mut_ptr(), element);
                self.set_len(len + 1);
            }
        }
    }
}

impl<T> Deref for Vec<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.as_ptr(), self.len()) }
    }
}

impl<T> DerefMut for Vec<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.as_mut_ptr(), self.len()) }
    }
}

impl<T> Default for Vec<T> {
    #[inline]
    fn default() -> Self {
        Vec::new()
    }
}

impl<T> AsRef<Vec<T>> for Vec<T> {
    #[inline]
    fn as_ref(&self) -> &Vec<T> {
        self
    }
}

impl<T> AsMut<Vec<T>> for Vec<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut Vec<T> {
        self
    }
}

impl<T> AsRef<[T]> for Vec<T> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        self
    }
}

impl<T> AsMut<[T]> for Vec<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut [T] {
        self
    }
}

impl<T> IntoIterator for Vec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        unsafe {
            let mut m = mem::ManuallyDrop::new(self);
            let ptr = m.as_mut_ptr();
            let end: *const T = if mem::size_of::<T>() == 0 {
                ptr.wrapping_offset(m.len() as isize)
            } else {
                ptr.add(m.len())
            };
            let cap = m.buf.capacity();
            IntoIter {
                buf: NonNull::new_unchecked(ptr),
                phantom: PhantomData,
                cap,
                ptr,
                end,
            }
        }
    }
}

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        let v = *self;
    }
}

impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T> FromIterator<T> for Vec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iterator = iter.into_iter();

        // Light optimization taken from SpecFromIterNested::from_iter.
        // Determines initial capacity using a size hint.
        let mut vector = match iterator.next() {
            None => Vec::new(),
            Some(element) => {
                let (lower, _) = iterator.size_hint();
                let initial_capacity =
                    cmp::max(RawVec::<T>::MIN_NON_ZERO_CAP, lower.saturating_add(1));
                let mut vector = Vec::with_capacity(initial_capacity);
                unsafe {
                    ptr::write(vector.as_mut_ptr(), element);
                    vector.set_len(1);
                }
                vector
            }
        };

        vector.extend(iterator);
        vector
    }
}

impl<T: Copy> From<&[T]> for Vec<T> {
    fn from(value: &[T]) -> Self {
        to_vec_copy(value)
    }
}

impl<T: Copy> From<&mut [T]> for Vec<T> {
    fn from(value: &mut [T]) -> Self {
        to_vec_copy(value)
    }
}

impl<T> From<Box<[T]>> for Vec<T> {
    fn from(value: Box<[T]>) -> Self {
        box_into_vec(value)
    }
}

impl From<&str> for Vec<u8> {
    fn from(value: &str) -> Self {
        From::from(value.as_bytes())
    }
}

impl<T> Drop for Vec<T> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            let p: *mut [T] = ptr::slice_from_raw_parts_mut(self.as_mut_ptr(), self.len());
            ptr::drop_in_place(p);
        }
    }
}

impl<T, I: SliceIndex<[T]>> Index<I> for Vec<T> {
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        Index::index(&**self, index)
    }
}

impl<T, I: SliceIndex<[T]>> IndexMut<I> for Vec<T> {
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        IndexMut::index_mut(&mut **self, index)
    }
}

fn to_vec_copy<T: Copy>(s: &[T]) -> Vec<T> {
    let mut v = Vec::with_capacity(s.len());
    unsafe {
        s.as_ptr().copy_to_nonoverlapping(v.as_mut_ptr(), s.len());
        v.set_len(s.len());
    }
    v
}

fn box_into_vec<T>(b: Box<[T]>) -> Vec<T> {
    let len = b.len();
    let b = Box::into_raw(b);
    Vec::from_raw_parts(b as *mut T, len, len)
}
