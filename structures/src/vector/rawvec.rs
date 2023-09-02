use std::alloc::handle_alloc_error;
use std::alloc::Layout;
use std::ptr::NonNull;
use std::{alloc, cmp, mem};

use TryReserveError::*;

pub enum TryReserveError {
    CapacityOverflow,
    AllocError { layout: alloc::Layout },
}

enum AllocInit {
    Uninitialized,
    Zeroed,
}

pub struct RawVec<T> {
    ptr: NonNull<T>,
    cap: usize,
}

unsafe impl<T: Send> Send for RawVec<T> {}
unsafe impl<T: Sync> Sync for RawVec<T> {}

impl<T> RawVec<T> {
    pub(crate) const MIN_NON_ZERO_CAP: usize = match mem::size_of::<T>() {
        d if d == 1 => 8,
        d if d <= 1024 => 4,
        _ => 1,
    };

    pub const NEW: Self = Self::new();

    pub const fn new() -> Self {
        Self {
            ptr: NonNull::<T>::dangling(),
            cap: 0,
        }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self::allocate_new(capacity, AllocInit::Uninitialized)
    }

    #[inline]
    pub fn with_capacity_zeroed(capacity: usize) -> Self {
        Self::allocate_new(capacity, AllocInit::Zeroed)
    }

    #[inline]
    pub fn ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }

    #[inline(always)]
    pub fn capacity(&self) -> usize {
        if is_zst::<T>() {
            usize::MAX as usize
        } else {
            self.cap
        }
    }

    fn layout(&self) -> Option<Layout> {
        if is_zst::<T>() || self.cap == 0 {
            return None;
        }

        // SAFETY: this memory is already allocated and ensured to be valid by Layout::array
        assert_eq!(mem::size_of::<T>() % mem::align_of::<T>(), 0);
        unsafe {
            let align = mem::align_of::<T>();
            let size = mem::size_of::<T>() * self.cap;
            let layout = Layout::from_size_align_unchecked(size, align);
            Some(layout)
        }
    }

    fn memory(&self) -> Option<(*mut u8, Layout)> {
        self.layout().map(|layout| (self.ptr() as *mut u8, layout))
    }

    #[inline(always)]
    fn handle_zst_overflow(&self) -> Result<(), TryReserveError> {
        if is_zst::<T>() {
            return Err(CapacityOverflow);
        } else {
            Ok(())
        }
    }

    fn unchecked_set_ptr_and_cap(&mut self, ptr: *mut u8, cap: usize) {
        // SAFETY: ptr must be non-null and valid
        self.ptr = unsafe { NonNull::new_unchecked(ptr as *mut T) };
        self.cap = cap;
    }

    fn try_allocate_new(capacity: usize, init: AllocInit) -> Result<Self, TryReserveError> {
        if is_zst::<T>() || capacity == 0 {
            return Ok(Self::new());
        }

        let layout = Layout::array::<T>(capacity).map_err(|_| CapacityOverflow)?;
        // new layout allocation should not exceed isize::MAX bytes
        if layout.size() > isize::MAX as usize {
            return Err(CapacityOverflow);
        }

        let ptr = match init {
            AllocInit::Uninitialized => unsafe { alloc::alloc(layout) },
            AllocInit::Zeroed => unsafe { alloc::alloc_zeroed(layout) },
        };

        // if an allocation error occurred, pointer would be null
        if ptr.is_null() {
            Err(AllocError { layout })
        } else {
            Ok(Self {
                ptr: unsafe { NonNull::new_unchecked(ptr as *mut T) },
                cap: capacity,
            })
        }
    }

    fn allocate_new(capacity: usize, init: AllocInit) -> Self {
        handle_reserve_unwrap(Self::try_allocate_new(capacity, init))
    }

    fn checked_alloc_cap(&mut self, cap: usize) -> Result<(), TryReserveError> {
        // caller should ensure that capacity doesn't overflow usize
        let layout = Layout::array::<T>(cap).map_err(|_| CapacityOverflow)?;
        // new layout allocation should not exceed isize::MAX bytes
        if layout.size() > isize::MAX as usize {
            return Err(CapacityOverflow);
        }

        // get pointer to allocated/reallocated memory
        // SAFETY: layout and size are ensured to be valid by Layout::array
        let ptr = if let Some(old_layout) = self.layout() {
            unsafe { alloc::realloc(self.ptr() as *mut u8, old_layout, layout.size()) }
        } else {
            unsafe { alloc::alloc(layout) }
        };

        // if an allocation error occurred, pointer would be null
        if ptr.is_null() {
            Err(AllocError { layout })
        } else {
            Ok(self.unchecked_set_ptr_and_cap(ptr, cap))
        }
    }

    fn needs_to_grow(&self, len: usize, additional: usize) -> bool {
        additional > self.capacity().wrapping_sub(len)
    }

    #[inline(never)]
    fn grow(&mut self, len: usize, additional: usize) -> Result<(), TryReserveError> {
        // handle zero sized type
        self.handle_zst_overflow()?;

        // calculate capacity based on additional required
        // cap <= isize::MAX, so it doesn't overflow usize
        let required_cap = len.checked_add(additional).ok_or(CapacityOverflow)?;
        let cap = cmp::max(self.cap * 2, required_cap);
        let cap = cmp::max(Self::MIN_NON_ZERO_CAP, cap);

        // allocate given capacity
        self.checked_alloc_cap(cap)
    }

    #[inline(never)]
    pub fn reserve_for_push(&mut self, len: usize) {
        handle_reserve(self.grow(len, 1));
    }

    #[inline]
    pub fn reserve(&mut self, len: usize, additional: usize) {
        if self.needs_to_grow(len, additional) {
            handle_reserve(self.grow(len, additional));
        }
    }

    pub fn try_reserve(&mut self, len: usize, additional: usize) -> Result<(), TryReserveError> {
        if self.needs_to_grow(len, additional) {
            self.grow(len, additional)
        } else {
            Ok(())
        }
    }

    #[inline(never)]
    fn grow_exact(&mut self, len: usize, additional: usize) -> Result<(), TryReserveError> {
        // handle zero sized type
        self.handle_zst_overflow()?;

        // check capacity
        let cap = len.checked_add(additional).ok_or(CapacityOverflow)?;

        // allocate given capacity
        self.checked_alloc_cap(cap)
    }

    pub fn reserve_exact(&mut self, len: usize, additional: usize) {
        handle_reserve(self.try_reserve_exact(len, additional));
    }

    pub fn try_reserve_exact(
        &mut self,
        len: usize,
        additional: usize,
    ) -> Result<(), TryReserveError> {
        if self.needs_to_grow(len, additional) {
            self.grow_exact(len, additional)
        } else {
            Ok(())
        }
    }

    pub fn from_raw_parts(value: *mut T, capacity: usize) -> RawVec<T> {
        RawVec {
            ptr: unsafe { NonNull::new_unchecked(value) },
            cap: capacity,
        }
    }
}

impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        if let Some((ptr, layout)) = self.memory() {
            unsafe { alloc::dealloc(ptr, layout) };
        }
    }
}

fn handle_reserve_unwrap<T>(result: Result<T, TryReserveError>) -> T {
    match result {
        Err(CapacityOverflow) => panic!("capacity overflow"),
        Err(AllocError { layout }) => handle_alloc_error(layout),
        Ok(value) => value,
    }
}

fn handle_reserve<T>(result: Result<T, TryReserveError>) {
    handle_reserve_unwrap(result);
}

#[inline(always)]
const fn is_zst<T>() -> bool {
    mem::size_of::<T>() == 0
}
