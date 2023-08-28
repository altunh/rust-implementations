use std::marker::PhantomData;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;
use std::{ptr, slice};

use super::iter::IntoIter;
use super::rawvec::RawVec;

pub struct Vec<T> {
    buf: RawVec<T>,
    len: usize,
}

impl<T> Vec<T> {
    pub const fn new() -> Self {
        Self {
            buf: RawVec::NEW,
            len: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buf: RawVec::with_capacity(capacity),
            len: 0,
        }
    }

    pub fn with_capacity_zeroed(capacity: usize) -> Self {
        Self {
            buf: RawVec::with_capacity_zeroed(capacity),
            len: 0,
        }
    }

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
}

impl<T> Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.as_ptr(), self.len()) }
    }
}

impl<T> DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.as_mut_ptr(), self.len()) }
    }
}

impl<T> Default for Vec<T> {
    fn default() -> Self {
        Vec::new()
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
