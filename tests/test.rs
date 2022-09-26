use arref::array_mut_ref;

#[test]
fn test_borrow_mut() {
    let mut arr = vec![1, 2, 3];
    let (a, b) = array_mut_ref!(&mut arr, [1, 2]);
    assert_eq!(*a, 2);
    assert_eq!(*b, 3);
}

#[test]
fn should_panic_if_borrow_the_same_element_mutably() {
    let result = std::panic::catch_unwind(|| {
        let mut arr = vec![1, 2, 3];
        let (_, _) = array_mut_ref!(&mut arr, [1, 1]);
    });
    assert!(result.is_err())
}
