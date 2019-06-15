#![cfg_attr(not(feature = "std"), no_std)]
#![feature(const_fn)]
#![feature(const_generics)]
#![feature(maybe_uninit_ref)]
#![feature(trusted_len)]
#![feature(try_trait)]

#![doc(html_root_url = "https://docs.rs/array-helpers/0.0.1")]

//! Basic length-generic methods on arrays.
//!
//! This crate provides utilities for initializing, consuming, and transforming arrays.
//! All methods are length-generic.
//!
//! This crate uses the extremely new `const_generics` feature.
//! As such, it requires nightly Rust.
//!
//! # Rationale
//!
//! It is difficult to work with large arrays in Rust, especially if their elements are non-Copy.
//! Initialization either doesn't work...
//! ```compile_fail
//! // the trait bound `std::vec::Vec<usize>: std::marker::Copy` is not satisfied
//! let arr: [Vec<usize>; 1000] = [Vec::new(); 1000];
//! ```
//! ```compile_fail
//! let arr: [Vec<usize>; 1000];
//! for item in arr.iter_mut() {
//!     *item = Vec::new();
//! }
//! ```
//! or is tedious...
//! ```
//! // borrow of possibly uninitialized variable: `arr`
//! let arr: [Vec<usize>; 1000] = [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];
//! ```
//! or unsafe ...
//! ```
//! let arr: [Vec<usize>; 1000] = unsafe {
//!     use std::mem::MaybeUninit;
//!     let mut a = std::mem::transmute::<MaybeUninit<[Vec<usize>; 1000]>, [MaybeUninit<Vec<usize>>; 1000]>(MaybeUninit::uninit());
//!     for item in a.iter_mut() {
//!         *item = MaybeUninit::new(Vec::new());
//!     }
//!     std::mem::transmute::<[MaybeUninit<Vec<usize>>; 1000], [Vec<usize>; 1000]>(a)
//! };
//! ```
//!
//! This crate fixes that. With this crate, initialization is easy.
//! ```
//! let arr: [Vec<usize>; 1000] = array_helpers::new(|_| Vec::new());
//! ```
//!
//! ## Why doesn't this already exist?
//!
//! `array_helpers::new`, and the other array_helpers methods, are generic over the length of the array.
//! This only became possible very recently.
//!
//! I expect these functions, or functions similar to them, to eventually end up in `core` and `std`. But that could take a long time.
//!
//! # Examples
//!
//! ## Basic Usage
//!
//! ```
//! # use array_helpers::*;
//! let arr: [[usize; 3]; 2] = array_helpers::new(|i| array_helpers::new(|j| 3*i + j));
//! assert_eq!(arr, [[0,1,2],[3,4,5]]);
//!
//! let arr = arr.transpose();
//! assert_eq!(arr, [[0,3],[1,4],[2,5]]);
//!
//! let arr = arr.map(|[a,b]| (a,b));
//! assert_eq!(arr, [(0,3),(1,4),(2,5)]);
//!
//! let (arr1, arr2) = arr.unzip();
//! assert_eq!(arr1, [0,1,2]);
//! assert_eq!(arr2, [3,4,5]);
//! ```
//!
//!
//! ## Small Example
//! ```
//! # use array_helpers::*;
//! #[derive(Debug,Clone)]
//! enum OctTree<T> {
//!     Leaf(T),
//!     Branch(Box<[Self;8]>),
//! }
//! 
//! # #[cfg(not(feature = "std"))]
//! # let tree: OctTree<usize> = OctTree::Branch(Box::new(new(OctTree::Leaf)));
//! # #[cfg(feature = "std")]
//! let tree: OctTree<usize> = OctTree::Branch(new_boxed(OctTree::Leaf));
//! 
//! impl<T> OctTree<T> {
//!     fn map<U>(self, mut f: impl FnMut(T) -> U) -> OctTree<U> {
//!         match self {
//!             OctTree::Leaf(t) => OctTree::Leaf(f(t)),
//!             OctTree::Branch(subtrees) => {
//!                 let mut closure: Box<dyn FnMut(T) -> U> = Box::new(f);
//!                 OctTree::Branch(Box::new(subtrees.map(|subtree| subtree.map(&mut closure))))
//!             }
//!         }
//!     }
//! }
//!
//! println!("{:?}", tree.map(|x| x*x))
//! ```
//!
//! ## Bigger example
//!
//! [Implementing matrices]
//!
//! # Warning
//!
//! `warning: the feature 'const_generics' is incomplete and may cause the compiler to crash`
//!
//! # Reference
//!
//! Most of The methods in this crate can be found inside the provided traits,
//! as this is the only way to add methods to `[T;N]`.
//! Here is a list of all provided methods:
//!
//! | Function      | Arguments         | Result                    |
//! |---------------|-------------------|---------------------------|
//! | [`new`]       | `usize -> T`      | `[T;N]`                   |
//! | [`new_boxed`] | `usize -> T`      | `Box<[T;N]>`              |
//! | [`into_iter`] | `[T;N]`           | `impl Iterator<Item = T>` |
//! | [`map`]       | `[T;N]`, `T -> U` | `[U;N]`                   |
//! | [`zip`]       | `[T;N]`, `[U;N]`  | `[(T,U);N]`               |
//! | [`unzip`]     | `[(T,U);N]`       | `[T;N]`, `[U;N]`          |
//! | [`transpose`] | `[[T;M];N]`       | `[[T;N];M]`               |
//!
//!
//! [`new`]: ./fn.new.html
//! [`new_boxed`]: ./fn.new_boxed.html
//! [`into_iter`]: ./trait.Array.html#tymethod.into_iter
//! [`map`]: ./trait.Array.html#tymethod.map
//! [`zip`]: ./trait.Array.html#tymethod.zip
//! [`unzip`]: ./trait.ArrayUnzip.html#tymethod.unzip
//! [`transpose`]: ./trait.ArrayTranspose.html#tymethod.transpose
//! [Implementing matrices]: ./example/index.html

use core::mem::MaybeUninit;

mod utils;
use crate::utils::*;
mod into_iter;
pub use into_iter::IntoIter;

pub mod example;

/// Takes a closure and creates an array by calling that closure on each index.
///
/// # Example
///
/// Basic usage:
///
/// ```
/// # use array_helpers::*;
/// let arr: [usize; 5] = array_helpers::new(|n| n * n);
/// assert_eq!(arr, [0, 1, 4, 9, 16]);
/// ```
pub fn new<T, const N: usize>(mut f: impl FnMut(usize) -> T) -> [T;N] {
    let mut arr: [MaybeUninit<T>; N] = push_maybe_uninit(MaybeUninit::uninit());
    for (i, item) in arr.iter_mut().enumerate() {
        *item = MaybeUninit::new(f(i));
    }
    unsafe {
        pull_maybe_uninit(arr).assume_init()
    }
}

#[cfg(feature = "std")]
/// Takes a closure and creates an array by calling that closure on each index.
///
/// The array is allocated directly on the heap.
///
/// # Example
///
/// Basic usage:
///
/// ```
/// # use array_helpers::*;
/// // Will not stack overflow
/// let arr: Box<[u64; 10_000_000]> = array_helpers::new_boxed(|n| (n as u64) * (n as u64));
/// assert_eq!(arr[9_999_999], 99_999_980_000_001);
/// ```
pub fn new_boxed<T, const N: usize>(mut f: impl FnMut(usize) -> T) -> Box<[T;N]> {
    unsafe {
        let mut v: Vec<MaybeUninit<T>> = Vec::with_capacity(N);
        v.set_len(N);
        for i in 0..N {
            v[i] = MaybeUninit::new(f(i));
        }
        let raw_slice = Box::into_raw(v.into_boxed_slice());
        Box::from_raw(raw_slice as *mut [T; N])
    }
}




/// A trait that exists for the sole purpose of allowing more methods to be put on arrays.
///
/// These methods cannot be attached directly to the type `[T;N]`.
/// Trying to do so results in error [E0118].
///
/// [`Array<T, {N}>`] is implemented for, and only for, the type `[T;N]`.
///
/// [E0118]: https://doc.rust-lang.org/error-index.html#E0118
/// [`Array<T, {N}>`]: ./trait.Array.html
pub trait Array<T, const N: usize>: private::ArraySealed where
    // Self == [T;N],
{

    /// Creates an iterator from an array.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use array_helpers::*;
    /// let arr: [usize; 4] = [1, 2, 3, 4];
    /// let mut iter = arr.into_iter();
    /// assert_eq!(Some(1), iter.next());
    /// assert_eq!(Some(2), iter.next());
    /// assert_eq!(Some(3), iter.next());
    /// assert_eq!(Some(4), iter.next());
    /// assert_eq!(None, iter.next());
    /// ```
    fn into_iter(self) -> IntoIter<T, {N}>;
    /// Takes a closure and creates a new array by calling that closure on each element.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use array_helpers::*;
    /// assert_eq!([1,2,3,4].map(|x| x * x), [1, 4, 9, 16]);
    /// ```
    fn map<U>(self, f: impl FnMut(T) -> U) -> [U;N];
    /// Converts a pair of arrays into an array of pairs.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use array_helpers::*;
    /// let arr1 = [1,2,3,4];
    /// let arr2 = ['a','b','c','d'];
    /// assert_eq!(arr1.zip(arr2), [(1, 'a'), (2, 'b'), (3, 'c'), (4, 'd')]);
    /// ```
    fn zip<U>(self, other: [U;N]) -> [(T,U); N];
}

impl<T, const N: usize> Array<T, {N}> for [T;N] {
    fn into_iter(self) -> IntoIter<T, {N}> {
        into_iter::new(self)
    }
    fn map<U>(self, mut f: impl FnMut(T) -> U) -> [U;N] {
        let mut arr = push_maybe_uninit(MaybeUninit::new(self));
        unsafe {
            new(|i| f(extract(&mut arr, i)))
        }
    }
    fn zip<U>(self, other: [U;N]) -> [(T,U); N] {
        let mut arr1 = push_maybe_uninit(MaybeUninit::new(self));
        let mut arr2 = push_maybe_uninit(MaybeUninit::new(other));
        unsafe {
            new(|i| (extract(&mut arr1, i), extract(&mut arr2, i)))
        }
    }
}


/// A trait that exists for the sole purpose of allowing the unzip method to be put on arrays.
///
/// This method cannot be attached directly to the type `[(T,U);N]`.
/// Trying to do so results in error [E0118].
///
/// [`ArrayUnzip<T, U, {N}>`] is implemented for, and only for, the type `[(T,U);N]`.
///
/// [E0118]: https://doc.rust-lang.org/error-index.html#E0118
/// [`ArrayUnzip<T, U, {N}>`]: ./trait.ArrayUnzip.html
pub trait ArrayUnzip<T, U, const N: usize>: private::ArrayUnzipSealed where
    // Self == [(T,U);N],
{
    /// Converts an array of pairs into a pair of arrays.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use array_helpers::*;
    /// let arr = [(1, 'a'), (2, 'b'), (3, 'c'), (4, 'd')];
    /// assert_eq!(arr.unzip(), ([1,2,3,4], ['a','b','c','d']));
    /// ```
    fn unzip(self) -> ([T;N],[U;N]);
}

impl<T, U, const N: usize> ArrayUnzip<T,U,{N}> for [(T,U);N] {
    fn unzip(self) -> ([T;N],[U;N]) {
        let mut arr = push_maybe_uninit(MaybeUninit::new(self));
        let mut out1: [MaybeUninit<T>; N] = push_maybe_uninit(MaybeUninit::uninit());
        let mut out2: [MaybeUninit<U>; N] = push_maybe_uninit(MaybeUninit::uninit());
        unsafe {
            for i in 0..N {
                let (t,u) = extract(&mut arr, i);
                out1[i] = MaybeUninit::new(t);
                out2[i] = MaybeUninit::new(u);
            }
            (pull_maybe_uninit(out1).assume_init(), pull_maybe_uninit(out2).assume_init())
        }
    }
}

/// A trait that exists for the sole purpose of allowing the transpose method to be put on arrays.
///
/// This method cannot be attached directly to the type `[[T;M];N]`.
/// Trying to do so results in error [E0118].
///
/// [`ArrayUnzip<T, {M}, {N}>`] is implemented for, and only for, the type `[[T;M];N]`.
///
/// [E0118]: https://doc.rust-lang.org/error-index.html#E0118
/// [`ArrayUnzip<T, {M}, {N}>`]: ./trait.ArrayUnzip.html
pub trait ArrayTranspose<T, const M: usize, const N: usize>: private::ArrayTransposeSealed where
    // Self == [[T;M];N],
{
    /// Transposes a 2D array.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use array_helpers::*;
    /// let arr = [[1,2,3],[4,5,6]];
    /// assert_eq!(arr.transpose(), [[1, 4], [2, 5], [3, 6]]);
    /// ```
    fn transpose(self) -> [[T;N];M];
}

impl<T, const M: usize, const N: usize> ArrayTranspose<T, {M}, {N}> for [[T;M];N] {
    fn transpose(self) -> [[T;N];M] {
        let mut arr = push_maybe_uninit(MaybeUninit::new(self)).map(push_maybe_uninit);
        unsafe {
            new(|i| new(|j| extract(&mut arr[j], i)))
        }
    }
}

/// A private module, whose traits cannot be implemented by users of the crate.
/// The traits in the main module require these as supertraits, so they also cannot be implemented by users of this crate.
/// This ensures that the methods in this module only apply to arrays.
mod private {
    pub trait ArraySealed {}
    impl<T, const N: usize> ArraySealed for [T;N] {}

    pub trait ArrayUnzipSealed {}
    impl<T, U, const N: usize> ArrayUnzipSealed for [(T,U);N] {}

    pub trait ArrayTransposeSealed {}
    impl<T, const M: usize, const N: usize> ArrayTransposeSealed for [[T;M];N] {}
}
