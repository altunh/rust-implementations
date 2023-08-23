// use structures::pointer::UnsafeCell;
use std::cell::UnsafeCell;

#[test]
fn unsafe_cell_book() {
    let x: UnsafeCell<i32> = 42.into();
    let (p1, p2) = (&x, &x);

    unsafe {
        let p1_exclusive = &mut *p1.get();
        let p2_shared = &*p2.get();
        *p1_exclusive += 27;
        assert_eq!(*p2_shared, 27 + 42);
    }

    unsafe {
        assert_eq!(*p2.get(), 27 + 42);
    }
}

#[test]
fn smoketest_unsafe_cell() {
    let mut x = UnsafeCell::new(10);
    let ref_mut = &mut x;
    unsafe {
        // The asserts are repeated in order to ensure that `get()`
        // is non-mutating.
        assert_eq!(*ref_mut.get(), 10);
        assert_eq!(*ref_mut.get(), 10);
        *ref_mut.get_mut() += 5;
        assert_eq!(*ref_mut.get(), 15);
        assert_eq!(*ref_mut.get(), 15);
        assert_eq!(x.into_inner(), 15);
    }
}

#[test]
fn unsafe_cell_raw_get() {
    let x = UnsafeCell::new(10);
    let ptr = &x as *const UnsafeCell<i32>;
    unsafe {
        // The asserts are repeated in order to ensure that `raw_get()`
        // is non-mutating.
        assert_eq!(*UnsafeCell::raw_get(ptr), 10);
        assert_eq!(*UnsafeCell::raw_get(ptr), 10);
        *UnsafeCell::raw_get(ptr) += 5;
        assert_eq!(*UnsafeCell::raw_get(ptr), 15);
        assert_eq!(*UnsafeCell::raw_get(ptr), 15);
        assert_eq!(x.into_inner(), 15);
    }
}

#[test]
fn unsafe_cell_unsized() {
    let cell: &UnsafeCell<[i32]> = &UnsafeCell::new([1, 2, 3]);
    {
        let val: &mut [i32] = unsafe { &mut *cell.get() };
        val[0] = 4;
        val[2] = 5;
    }
    let comp: &mut [i32] = &mut [4, 2, 5];
    assert_eq!(unsafe { &mut *cell.get() }, comp);
}

