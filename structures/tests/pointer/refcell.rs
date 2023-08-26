// use std::cell::{RefCell, Ref, RefMut};
use structures::pointer::{Ref, RefCell, RefMut};

#[test]
fn ref_and_refmut_have_sensible_show() {
    let refcell = RefCell::new("foo");

    let refcell_refmut = refcell.borrow_mut();
    assert_eq!(format!("{refcell_refmut}"), "foo"); // Display
    assert!(format!("{refcell_refmut:?}").contains("foo")); // Debug
    drop(refcell_refmut);

    let refcell_ref = refcell.borrow();
    assert_eq!(format!("{refcell_ref}"), "foo"); // Display
    assert!(format!("{refcell_ref:?}").contains("foo")); // Debug
    drop(refcell_ref);
}

#[test]
fn double_imm_borrow() {
    let x = RefCell::new(0);
    let _b1 = x.borrow();
    x.borrow();
}

#[test]
fn no_mut_then_imm_borrow() {
    let x = RefCell::new(0);
    let _b1 = x.borrow_mut();
    assert!(x.try_borrow().is_err());
}

#[test]
fn no_imm_then_borrow_mut() {
    let x = RefCell::new(0);
    let _b1 = x.borrow();
    assert!(x.try_borrow_mut().is_err());
}

#[test]
fn no_double_borrow_mut() {
    let x = RefCell::new(0);
    assert!(x.try_borrow().is_ok());
    let _b1 = x.borrow_mut();
    assert!(x.try_borrow().is_err());
}

#[test]
fn imm_release_borrow_mut() {
    let x = RefCell::new(0);
    {
        let _b1 = x.borrow();
    }
    x.borrow_mut();
}

#[test]
fn mut_release_borrow_mut() {
    let x = RefCell::new(0);
    {
        let _b1 = x.borrow_mut();
    }
    x.borrow();
}

#[test]
fn double_borrow_single_release_no_borrow_mut() {
    let x = RefCell::new(0);
    let _b1 = x.borrow();
    {
        let _b2 = x.borrow();
    }
    assert!(x.try_borrow().is_ok());
    assert!(x.try_borrow_mut().is_err());
}

#[test]
#[should_panic]
fn discard_doesnt_unborrow() {
    let x = RefCell::new(0);
    let _b = x.borrow();
    let _ = _b;
    let _b = x.borrow_mut();
}

#[test]
#[should_panic]
fn refcell_swap_borrows() {
    let x = RefCell::new(0);
    let _b = x.borrow();
    let y = RefCell::new(1);
    x.swap(&y);
}

#[test]
#[should_panic]
fn refcell_replace_borrows() {
    let x = RefCell::new(0);
    let _b = x.borrow();
    x.replace(1);
}

#[test]
fn refcell_format() {
    let name = RefCell::new("rust");
    let what = RefCell::new("rocks");
    let msg = format!("{name} {}", &*what.borrow(), name = &*name.borrow());
    assert_eq!(msg, "rust rocks".to_string());
}

#[test]
fn refcell_default() {
    let cell: RefCell<u64> = Default::default();
    assert_eq!(0, *cell.borrow());
}

#[test]
fn refcell_unsized() {
    let cell: &RefCell<[i32]> = &RefCell::new([1, 2, 3]);
    {
        let b = &mut *cell.borrow_mut();
        b[0] = 4;
        b[2] = 5;
    }
    let comp: &mut [i32] = &mut [4, 2, 5];
    assert_eq!(&*cell.borrow(), comp);
}

#[test]
fn refcell_ref_coercion() {
    let cell: RefCell<[i32; 3]> = RefCell::new([1, 2, 3]);
    {
        let mut cellref: RefMut<'_, [i32; 3]> = cell.borrow_mut();
        cellref[0] = 4;
        let mut coerced: RefMut<'_, [i32]> = cellref;
        coerced[2] = 5;
    }
    {
        let comp: &mut [i32] = &mut [4, 2, 5];
        let cellref: Ref<'_, [i32; 3]> = cell.borrow();
        assert_eq!(&*cellref, comp);
        let coerced: Ref<'_, [i32]> = cellref;
        assert_eq!(&*coerced, comp);
    }
}

#[test]
fn as_ptr() {
    let r1: RefCell<usize> = RefCell::new(0);
    *r1.borrow_mut() = 1;
    assert_eq!(1, unsafe { *r1.as_ptr() });

    let r2: RefCell<usize> = RefCell::new(0);
    unsafe {
        *r2.as_ptr() = 1;
    }
    assert_eq!(1, *r2.borrow());
}
