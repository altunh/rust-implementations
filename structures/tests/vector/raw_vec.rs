use structures::vector::RawVec;

#[test]
fn reserve_push() {
    let mut rv = RawVec::<usize>::new();
    rv.reserve_for_push(0);
    assert_eq!(rv.capacity(), 4);
    rv.reserve_for_push(1);
    assert_eq!(rv.capacity(), 8);
    rv.reserve_for_push(2);
    assert_eq!(rv.capacity(), 16);
}

#[test]
fn reserve_exact() {
    let mut rv = RawVec::<usize>::new();
    rv.reserve_exact(0, 9);
    assert_eq!(rv.capacity(), 9);
    rv.reserve_for_push(9);
    assert_eq!(rv.capacity(), 18);
    rv.reserve_for_push(10);
    assert_eq!(rv.capacity(), 36);
    rv.reserve_exact(11, 30);
    assert_eq!(rv.capacity(), 41);
}
