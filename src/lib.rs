#![no_std]

use core::marker::PhantomData;

/// a trait which when implemented by some type states that the type's memory representation can be treated directly as a slice of
/// type `T`, with a length that is according to the `LENGTH` constant.
pub unsafe trait RecursiveArray<T>: Sized {
    /// the length of this array
    const LENGTH: usize;

    /// an empty recursive array.
    const EMPTY: EmptyRecursiveArray = EmptyRecursiveArray;

    /// returns an empty recursive array.
    fn empty() -> EmptyRecursiveArray {
        EmptyRecursiveArray
    }

    /// converts the given slice to a recursive array reference. this is a zero cost operation, which just casts the slice.
    ///
    /// # Panics
    ///
    /// this function panics if the length of the slice is not equal to `Self::LENGTH`.
    fn from_slice(slice: &[T]) -> &Self {
        if slice.len() != Self::LENGTH {
            panic!(
                "tried to convert a slice of length {} to a recursive array of length {}",
                slice.len(),
                Self::LENGTH,
            );
        }
        unsafe { &*slice.as_ptr().cast() }
    }

    /// converts the given mutable slice to a recursive array mutable reference. this is a zero cost operation, which just casts the slice.
    ///
    /// # Panics
    ///
    /// this function panics if the length of the slice is not equal to `Self::LENGTH`.
    fn from_mut_slice(slice: &mut [T]) -> &mut Self {
        if slice.len() != Self::LENGTH {
            panic!(
                "tried to convert a slice of length {} to a recursive array of length {}",
                slice.len(),
                Self::LENGTH,
            );
        }
        unsafe { &mut *slice.as_mut_ptr().cast() }
    }

    /// returns the elements of this array as a slice.
    fn as_slice(&self) -> &[T] {
        unsafe { core::slice::from_raw_parts(self as *const Self as *const T, Self::LENGTH) }
    }

    /// returns the elements of this array as a mutable slice.
    fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { core::slice::from_raw_parts_mut(self as *mut Self as *mut T, Self::LENGTH) }
    }

    /// appends an element to the back of this array.
    fn push_back(
        self,
        item: T,
    ) -> RecursiveArrayConcatenation<T, Self, RecursiveArraySingleItem<T>> {
        RecursiveArrayConcatenation::new(self, RecursiveArraySingleItem::new(item))
    }

    /// appends a recrusive array to the back of this array.
    fn append_back<R: RecursiveArray<T>>(
        self,
        array: R,
    ) -> RecursiveArrayConcatenation<T, Self, R> {
        RecursiveArrayConcatenation::new(self, array)
    }

    /// appends an element to the fron of this array.
    fn push_front(
        self,
        item: T,
    ) -> RecursiveArrayConcatenation<T, RecursiveArraySingleItem<T>, Self> {
        RecursiveArrayConcatenation::new(RecursiveArraySingleItem::new(item), self)
    }

    /// appends a recrusive array to the front of this array.
    fn append_front<R: RecursiveArray<T>>(
        self,
        array: R,
    ) -> RecursiveArrayConcatenation<T, R, Self> {
        RecursiveArrayConcatenation::new(array, self)
    }
}

/// a type used for comparing 2 const usizes for equality at compile time.
pub struct CompareConstUsizes<const A: usize, const B: usize>;

/// a trait which is used for representing the condition that 2 const usizes are equal at compile time, by applying it as a constraint
/// on the `CompareConstUsizes` type.
pub trait ConstUsizesEqual {}
impl<const N: usize> ConstUsizesEqual for CompareConstUsizes<N, N> {}

/// an empty recrusive array.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub struct EmptyRecursiveArray;
unsafe impl<T> RecursiveArray<T> for EmptyRecursiveArray {
    const LENGTH: usize = 0;
}

/// a recursive array with a single item.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct RecursiveArraySingleItem<T> {
    item: T,
}
unsafe impl<T> RecursiveArray<T> for RecursiveArraySingleItem<T> {
    const LENGTH: usize = 1;
}
impl<T> RecursiveArraySingleItem<T> {
    /// creates a new recrusive array with a single item.
    pub fn new(item: T) -> Self {
        Self { item }
    }
}

/// a recursive array which concatenates 2 recursive arrays.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
#[repr(C)]
pub struct RecursiveArrayConcatenation<T, A: RecursiveArray<T>, B: RecursiveArray<T>> {
    a: A,
    b: B,
    phantom: PhantomData<T>,
}
unsafe impl<T, A: RecursiveArray<T>, B: RecursiveArray<T>> RecursiveArray<T>
    for RecursiveArrayConcatenation<T, A, B>
{
    const LENGTH: usize = A::LENGTH + B::LENGTH;
}
impl<T, A: RecursiveArray<T>, B: RecursiveArray<T>> RecursiveArrayConcatenation<T, A, B> {
    /// creates a new recrusive array which concatenates the 2 given recursive arrays.
    pub fn new(a: A, b: B) -> Self {
        Self {
            a,
            b,
            phantom: PhantomData,
        }
    }
}

/// a recursive array wrapper which wraps a regular rust array (`[T; N]`) and allows it to be treated as a recursive array.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct RecursiveArrayArrayWrapper<const N: usize, T> {
    array: [T; N],
}
impl<const N: usize, T> RecursiveArrayArrayWrapper<N, T> {
    /// creates a new recursive array wrapper which wraps the given array.
    pub fn new(array: [T; N]) -> Self {
        Self { array }
    }
}
unsafe impl<const N: usize, T> RecursiveArray<T> for RecursiveArrayArrayWrapper<N, T> {
    const LENGTH: usize = N;
}

/// a macro for instantiating a recursive array with the given elements.
#[macro_export]
macro_rules! recursive_array {
    [] => {
        ::recursive_array::EmptyRecursiveArray
    };
    [$item: expr $(,)?] => {
        ::recursive_array::RecursiveArraySingleItem::new($item)
    };
    [$first_item: expr, $($item: expr),+] => {
        ::recursive_array::RecursiveArrayConcatenation::new(
            ::recursive_array::RecursiveArraySingleItem::new($first_item),
            ::recursive_array::recursive_array![$($item),+],
        )
    };
}

/// a macro for getting the type of a generic array with the given item type and size.
#[macro_export]
macro_rules! recursive_array_type_of_size {
    ($item_type: ty, $size: expr) => {
        ::recursive_array::RecursiveArrayArrayWrapper<{$size}, $item_type>
    };
}
