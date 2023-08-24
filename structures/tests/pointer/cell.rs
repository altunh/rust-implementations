use structures::pointer::Cell;

#[test]
fn smoketest_cell() {
    let x = Cell::new(10);
    assert_eq!(x, Cell::new(10));
    assert_eq!(x.get(), 10);
    x.set(20);
    assert_eq!(x, Cell::new(20));
    assert_eq!(x.get(), 20);

    let y = Cell::new((30, 40));
    assert_eq!(y, Cell::new((30, 40)));
    assert!(y > Cell::new((20, 30)));
    assert_eq!(y.get(), (30, 40));
}

#[test]
fn cell_is_dbg() {
    use std::fmt::Debug;

    let y = Cell::new((30, 40));
    let b = Box::new(y);
    let d = b as Box<dyn Debug>;
    eprintln!("{:?}", d);
}

/// From the `Cell<T>` doc example
#[test]
fn cell_is_mutable() {
    struct SomeStruct {
        regular_field: u8,
        special_field: Cell<u8>,
    }

    let my_struct = SomeStruct {
        regular_field: 0,
        special_field: Cell::new(1),
    };

    my_struct.special_field.set(100);
    assert_eq!(my_struct.special_field.get(), 100);
    assert_eq!(my_struct.regular_field, 0);

    // Compiler: my_struct is not declared as mutable.
    // my_struct.regular_field = 100;
}

#[test]
fn cell_has_sensible_show() {
    let x = Cell::new("foo bar");
    assert!(format!("{x:?}").contains(x.get()));

    x.set("baz qux");
    assert!(format!("{x:?}").contains(x.get()));
}

#[test]
fn cell_default() {
    let cell: Cell<u32> = Default::default();
    assert_eq!(0, cell.get());
}

#[test]
fn cell_consumed() {
    let cell = Cell::new(10);
    assert_eq!(10, cell.into_inner());
    // cell.set(20);
}

#[test]
fn cell_set() {
    let cell = Cell::new(10);
    cell.set(20);
    assert_eq!(20, cell.get());

    let cell = Cell::new("Hello".to_owned());
    cell.set("World".to_owned());
    assert_eq!("World".to_owned(), cell.into_inner());
}

#[test]
fn cell_replace() {
    let cell = Cell::new(10);
    assert_eq!(10, cell.replace(20));
    assert_eq!(20, cell.get());

    let cell = Cell::new("Hello".to_owned());
    assert_eq!("Hello".to_owned(), cell.replace("World".to_owned()));
    assert_eq!("World".to_owned(), cell.into_inner());
}

#[test]
fn cell_into_inner() {
    let cell = Cell::new(10);
    assert_eq!(10, cell.into_inner());

    let cell = Cell::new("Hello world".to_owned());
    assert_eq!("Hello world".to_owned(), cell.into_inner());
}

#[test]
fn cell_exterior() {
    #[derive(Copy, Clone)]
    #[allow(dead_code)]
    struct Point {
        x: isize,
        y: isize,
        z: isize,
    }

    fn f(p: &Cell<Point>) {
        assert_eq!(p.get().z, 12);
        p.set(Point {
            x: 10,
            y: 11,
            z: 13,
        });
        assert_eq!(p.get().z, 13);
    }

    let a = Point {
        x: 10,
        y: 11,
        z: 12,
    };
    let b = &Cell::new(a);
    assert_eq!(b.get().z, 12);
    f(b);
    assert_eq!(a.z, 12);
    assert_eq!(b.get().z, 13);
}

#[test]
fn cell_does_not_clone() {
    #[derive(Copy)]
    #[allow(dead_code)]
    struct Foo {
        x: isize,
    }

    impl Clone for Foo {
        fn clone(&self) -> Foo {
            panic!();
        }
    }

    let x = Cell::new(Foo { x: 22 });
    let _y = x.get();
    let _z = x.clone();
}

#[test]
fn as_ptr() {
    let c1: Cell<usize> = Cell::new(0);
    c1.set(1);
    assert_eq!(1, unsafe { *c1.as_ptr() });

    let c2: Cell<usize> = Cell::new(0);
    unsafe {
        *c2.as_ptr() = 1;
    }
    assert_eq!(1, c2.get());
}

#[test]
fn cell_update() {
    let x = Cell::new(10);

    assert_eq!(x.update(|x| x + 5), 15);
    assert_eq!(x.get(), 15);

    assert_eq!(x.update(|x| x / 3), 5);
    assert_eq!(x.get(), 5);
}