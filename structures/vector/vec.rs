use std::ptr;

use super::raw_vec::RawVec;

pub struct Vec<T> {
    buf: RawVec<T>,
    len: usize,
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        Self {
            buf: RawVec::new(),
            len: 0,
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
    pub(crate) unsafe fn set_len(&mut self, len: usize) {
        debug_assert!(len <= self.capacity());
        self.len = len;
    }

    pub fn push(&mut self, value: T) {
        // if length reached capacity, request an additional space of 1
        if self.len == self.capacity() {
            self.buf.reserve_for_push(self.len);
        }

        // compute and write to the end of the pointer
        // SAFETY: since here len < cap, Vec::grow ensures that offset is valid
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

        self.len -= 1;
        unsafe {
            let end = self.as_mut_ptr().add(self.len);
            Some(ptr::read(end))
        }
    }

    // pub fn insert(&mut self, index: usize, element: T) {
    //     todo!()
    // }

    // pub fn remove(&mut self, index: usize) -> T {
    //     todo!()
    // }

    // pub fn swap_remove(&mut self, index: usize) -> T {
    //     todo!()
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ZST;
    const UMAX: usize = usize::MAX;

    #[test]
    fn basic_test() {
        let mut v = Vec::<usize>::new();
        assert_eq!(v.capacity(), 0);
        v.push(0);
        assert_eq!(v.capacity(), 4);
        v.push(1);
        assert_eq!(v.len(), 2);
    }

    #[test]
    fn zst_test() {
        let mut v = Vec::<ZST>::new();
        assert_eq!((v.capacity(), v.len()), (UMAX, 0));
        v.push(ZST);
        assert_eq!((v.capacity(), v.len()), (UMAX, 1));
        for _ in 0..100 {
            v.push(ZST);
        }
        assert_eq!((v.capacity(), v.len()), (UMAX, 101));
    }

    #[test]
    #[should_panic]
    fn zst_overflow() {
        let mut v = Vec::<ZST>::new();
        unsafe { v.set_len(UMAX) };
        assert_eq!((v.capacity(), v.len()), (UMAX, UMAX));
        v.push(ZST);
    }

    #[test]
    fn cap_test() {
        let mut v = Vec::<usize>::new();
        for num in 0..16 {
            v.push(num);
        }
        assert_eq!((v.capacity(), v.len()), (16, 16));
        for num in 16..100 {
            v.push(num);
        }
        assert_eq!((v.capacity(), v.len()), (128, 100));
    }
}
