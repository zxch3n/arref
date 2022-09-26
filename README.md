Getting mutable references to two elements from the same array is hard. This
tiny lib provides method to make it easier.

[array_mut_ref!] checks whether the user borrows the same element at runtime.

```rust
use arref::array_mut_ref;
let mut arr = vec![1, 2, 3, 4];
let (a, b) = array_mut_ref!(&mut arr, [1, 2]);
assert_eq!(*a, 2);
assert_eq!(*b, 3);
let (a, b, c) = array_mut_ref!(&mut arr, [1, 2, 0]);
assert_eq!(*c, 1);

// ⚠️ The following code will panic. Because we borrow the same element twice.
// let (a, b) = array_mut_ref!(&mut arr, [1, 1]);
```

Alternatively, you can use [mut_twice]. It won't panic if you borrow the same
element twice. It'll return an `Err(&mut T)` instead.

```rust
use arref::mut_twice;
let mut arr = vec![1, 2, 3];
let (a, b) = mut_twice(&mut arr, 1, 2).unwrap();
assert_eq!(*a, 2);
assert_eq!(*b, 3);
let result = mut_twice(&mut arr, 1, 1);
assert!(result.is_err());
if let Err(v) = result {
    assert_eq!(*v, 2);
}
```
