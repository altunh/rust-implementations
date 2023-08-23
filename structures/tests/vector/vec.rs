use structures::vector::Vec;

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

// #[test]
// #[should_panic]
// fn zst_overflow() {
//     let mut v = Vec::<ZST>::new();
//     unsafe { v.set_len(UMAX) };
//     assert_eq!((v.capacity(), v.len()), (UMAX, UMAX));
//     v.push(ZST);
// }

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
