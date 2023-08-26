use std::{ptr::NonNull, marker::PhantomData};

pub struct IntoIter<T> {
    pub(super) buf: NonNull<T>,
    pub(super) phantom: PhantomData<T>,
    pub(super) cap: usize,
    pub(super) ptr: *const T,
    pub(super) end: *const T,
}

