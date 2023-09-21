#![no_std]

/// a trait which when implemented by some type states that the type's memory representation can be treated directly as a slice of
/// type `T`, with a length that is according to the `LENGTH` constant.
pub unsafe trait RecursiveArrayTrait<T> {
    /// the length of this array
    const LENGTH: usize;

    /// returns the elements of this array as a slice.
    fn as_slice(&self) -> &[T] {
        unsafe { core::slice::from_raw_parts(self as *const Self as *const T, Self::LENGTH) }
    }

    /// returns the elements of this array as a mutable slice.
    fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { core::slice::from_raw_parts_mut(self as *mut Self as *mut T, Self::LENGTH) }
    }
}

/// an empty recrusive array.
pub struct EmptyRecursiveArray;
unsafe impl<T> RecursiveArrayTrait<T> for EmptyRecursiveArray {
    const LENGTH: usize = 0;
}

/// a recursive array.
#[repr(C)]
pub struct RecursiveArray<T, R: RecursiveArrayTrait<T>> {
    recursion: R,
    item: T,
}
unsafe impl<T, R: RecursiveArrayTrait<T>> RecursiveArrayTrait<T> for RecursiveArray<T, R> {
    const LENGTH: usize = R::LENGTH + 1;
}
impl<T, R: RecursiveArrayTrait<T>> RecursiveArray<T, R> {
    /// an empty recursive array.
    pub const EMPTY: EmptyRecursiveArray = EmptyRecursiveArray;

    /// returns an empty recursive array.
    pub fn empty() -> EmptyRecursiveArray {
        EmptyRecursiveArray
    }

    /// pushes an item into this array.
    pub fn push(self, item: T) -> RecursiveArray<T, Self> {
        RecursiveArray {
            recursion: self,
            item,
        }
    }
}

/// a macro for instantiating a recursive array with the given elements.
#[macro_export]
macro_rules! recursive_array {
    () => {
        ::recursive_array::EmptyRecursiveArray
    };
    ($item: expr $(,)?) => {
        ::recursive_array::RecursiveArray {
            recursion: ::recursive_array::EmptyRecursiveArray,
            item: $item,
        }
    };
    ($($item: expr,)+ $last_item: expr $(,)?) => {
        ::recursive_array::RecursiveArray {
            recursion: ::recursive_array::recursive_array!($($item,)+),
            item: $last_item,
        }
    };
}
