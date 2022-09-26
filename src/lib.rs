//! Getting mutable references to two elements from the same array is hard.
//! This tiny lib provides method to make it easier.
//!
//! [array_mut_ref!] checks whether the user borrows the same element at runtime.
//!
//! ```rust
//! use arref::array_mut_ref;
//! let mut arr = vec![1, 2, 3, 4];
//! let (a, b) = array_mut_ref!(&mut arr, [1, 2]);
//! assert_eq!(*a, 2);
//! assert_eq!(*b, 3);
//! let (a, b, c) = array_mut_ref!(&mut arr, [1, 2, 0]);
//! assert_eq!(*c, 1);
//!
//! // ⚠️ The following code will panic. Because we borrow the same element twice.
//! // let (a, b) = array_mut_ref!(&mut arr, [1, 1]);
//! ```
//!
//! Alternatively, you can use [mut_twice]. It won't panic if you borrow the same element twice.
//! It'll return an `Err(&mut T)` instead.
//!
//! ```rust
//! use arref::mut_twice;
//! let mut arr = vec![1, 2, 3];
//! let (a, b) = mut_twice(&mut arr, 1, 2).unwrap();
//! assert_eq!(*a, 2);
//! assert_eq!(*b, 3);
//! let result = mut_twice(&mut arr, 1, 1);
//! assert!(result.is_err());
//! if let Err(v) = result {
//!     assert_eq!(*v, 2);
//! }
//! ```
//!

/// It checks whether borrowing the same element at runtime, if so it'll panic.
///
/// ```rust
/// use arref::array_mut_ref;
/// let mut arr = vec![1, 2, 3, 4];
/// let (a, b) = array_mut_ref!(&mut arr, [1, 2]);
/// assert_eq!(*a, 2);
/// assert_eq!(*b, 3);
/// let (a, b, c) = array_mut_ref!(&mut arr, [1, 2, 0]);
/// assert_eq!(*c, 1);
///
/// // ⚠️ The following code will panic. Because we borrow the same element twice.
/// // let (a, b) = array_mut_ref!(&mut arr, [1, 1]);
/// ```
#[macro_export]
macro_rules! array_mut_ref {
    ($arr:expr, [$a0:expr, $a1:expr]) => {{
        arref::array_mut_ref($arr, $a0, $a1)
    }};
    ($arr:expr, [$a0:expr, $a1:expr, $a2:expr]) => {{
        arref::array_mut_ref3($arr, $a0, $a1, $a2)
    }};
}

/// Get mutable references to two elements from the array.
///
/// If a0 and a1 point to the same element, it will return Err(&mut T).
/// ```rust
/// use arref::mut_twice;
/// let mut arr = vec![1, 2, 3];
/// let (a, b) = mut_twice(&mut arr, 1, 2).unwrap();
/// assert_eq!(*a, 2);
/// assert_eq!(*b, 3);
/// let result = mut_twice(&mut arr, 1, 1);
/// assert!(result.is_err());
/// if let Err(v) = result {
///     assert_eq!(*v, 2);
/// }
/// ```
#[inline]
pub fn mut_twice<T>(arr: &mut [T], a0: usize, a1: usize) -> Result<(&mut T, &mut T), &mut T> {
    if a0 == a1 {
        Err(&mut arr[a0])
    } else {
        unsafe {
            Ok((
                &mut *(&mut arr[a0] as *mut _),
                &mut *(&mut arr[a1] as *mut _),
            ))
        }
    }
}

#[doc(hidden)]
#[inline]
pub fn array_mut_ref3<T>(
    arr: &mut [T],
    a0: usize,
    a1: usize,
    a2: usize,
) -> (&mut T, &mut T, &mut T) {
    assert!(a0 != a1 && a1 != a2 && a0 != a2);
    // SAFETY: this is safe because we know there are not multiple mutable references to the same element
    unsafe {
        (
            &mut *(&mut arr[a0] as *mut _),
            &mut *(&mut arr[a1] as *mut _),
            &mut *(&mut arr[a2] as *mut _),
        )
    }
}

#[doc(hidden)]
#[inline]
pub fn array_mut_ref<T>(arr: &mut [T], a0: usize, a1: usize) -> (&mut T, &mut T) {
    assert!(a0 != a1);
    // SAFETY: this is safe because we know a0 != a1
    unsafe {
        (
            &mut *(&mut arr[a0] as *mut _),
            &mut *(&mut arr[a1] as *mut _),
        )
    }
}
