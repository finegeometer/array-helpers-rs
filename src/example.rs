//! # Example: Implementing Matrices
//!
//! ```
//! #![feature(const_generics)]
//! # use array_helpers::*;
//! 
//! #[derive(Copy, Clone)]
//! /// A linear transformation, implemented as a matrix.
//! struct Transform<const A: usize, const B: usize>([[f64;B];A]);
//! 
//! fn identity<const N: usize>() -> Transform<{N},{N}> {
//!     Transform(new(|j| new(|i| if i == j {1.} else {0.})))
//! }
//! 
//! impl<const A: usize, const B: usize, const C: usize> core::ops::Mul<Transform<{A},{B}>> for Transform<{B},{C}> {
//!     type Output = Transform<{A},{C}>;
//!     fn mul(self, other: Transform<{A},{B}>) -> Transform<{A},{C}> {
//!         Transform(other.0.map(|u| {
//!             self.0
//!                 .transpose()
//!                 .map(|v| u.zip(v).into_iter().map(|(a, b)| a * b).sum())
//!         }))
//!     }
//! }
//! 
//! 
//! 
//! 
//! impl<const A: usize, const B: usize> PartialEq for Transform<{A},{B}> {
//!     fn eq(&self, other: &Self) -> bool {
//!         for i in 0..A {
//!             for j in 0..B {
//!                 if self.0[i][j] != other.0[i][j] {
//!                     return false;
//!                 }
//!             }
//!         }
//!         true
//!     }
//! }
//! 
//! assert!(identity::<3>() == Transform::<3,3>([[1.0,0.0,0.0],[0.0,1.0,0.0],[0.0,0.0,1.0]]));
//! 
//! let m1 = Transform::<1,3>([[1.,2.,3.]]);
//! let m2 = Transform::<3,1>([[1.],[2.],[3.]]);
//! 
//! assert!(m2 * m1 == Transform::<1,1>([[14.]]));
//! assert!(m1 * m2 == Transform::<3,3>([[1.,2.,3.],[2.,4.,6.],[3.,6.,9.]]));
//! ```
