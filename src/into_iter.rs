use core::iter::*;
use core::mem::MaybeUninit;
use crate::utils::*;


/// Array Iterator
///
/// This struct is created by the [`into_iter`] method.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// // First, we create an array:
/// let array: [usize; 4] = [0, 1, 2, 3];
///
/// // Then, we iterate over it.
/// for element in array.into_iter() {
/// 	println!("{}", element);
/// }
/// ```
///
/// [`into_iter`]: ./trait.Array.html#tymethod.into_iter
pub struct IntoIter<T, const N: usize> {
	/// arr[0..ptr] is uninitialized. arr[ptr..N] are the remaining elements in the iterator.
	arr: [MaybeUninit<T>; N],
	/// The index of the next item in the iterator.
	ptr: usize,
}

/// Create an IntoIter from a full array.
pub fn new<T, const N: usize>(arr: [T;N]) -> IntoIter<T,{N}> {
	IntoIter {
		arr: push_maybe_uninit(MaybeUninit::new(arr)),
		ptr: 0,
	}
}

impl<T, const N: usize> Drop for IntoIter<T,{N}> {
	fn drop(&mut self) {
		// Exhaust the iterator. Afterwards, the entire array will be uninitialized, and can be safely dropped without memory leakage.
		self.for_each(core::mem::drop);
	}
}

impl<T, const N: usize> Iterator for IntoIter<T,{N}> {
	type Item = T;
	fn next(&mut self) -> Option<T> {
		if self.ptr < N {
			let out = unsafe {extract(&mut self.arr, self.ptr)};
			self.ptr += 1;
			Some(out)
		} else {
			None
		}
	}
	// Implemented in terms of ExactSizeIterator.
	fn size_hint(&self) -> (usize, Option<usize>) {
		let out = self.len();
		(out, Some(out))
	}
	fn try_fold<B,F,R>(&mut self, mut state: B, mut f: F) -> R where
		F: FnMut(B, T) -> R,
		R: core::ops::Try<Ok = B>,
	{
		while self.ptr < N {
			let item = unsafe {extract(&mut self.arr, self.ptr)};
			self.ptr += 1;
			state = f(state, item)?;
		}
		R::from_ok(state)
	}
}

impl<T, const N: usize> ExactSizeIterator for IntoIter<T,{N}> {
	fn len(&self) -> usize {
		N - self.ptr
	}
}
impl<T, const N: usize> FusedIterator for IntoIter<T,{N}> {}
unsafe impl<T, const N: usize> TrustedLen for IntoIter<T,{N}> {}


impl<T: Clone, const N: usize> Clone for IntoIter<T,{N}> {
	fn clone(&self) -> Self {
		let mut arr = push_maybe_uninit(MaybeUninit::uninit());
		// Clone only the initialized indices.
		unsafe {
			for i in self.ptr..N {
				arr[i] = MaybeUninit::new(self.arr[i].get_ref().clone())
			}
		}
		Self { arr, ptr: self.ptr }
	}
}

impl<T: PartialEq, const N: usize> PartialEq for IntoIter<T,{N}> {
	fn eq(&self, other: &Self) -> bool {
		unsafe {
			let slice1 = core::mem::transmute::<&[MaybeUninit<T>], &[T]>(&self.arr[self.ptr..N]);
			let slice2 = core::mem::transmute::<&[MaybeUninit<T>], &[T]>(&other.arr[other.ptr..N]);
			slice1.eq(slice2)
		}
	}
}

impl<T: Eq, const N: usize> Eq for IntoIter<T,{N}> {}

impl<T: PartialOrd, const N: usize> PartialOrd for IntoIter<T,{N}> {
	fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
		unsafe {
			let slice1 = core::mem::transmute::<&[MaybeUninit<T>], &[T]>(&self.arr[self.ptr..N]);
			let slice2 = core::mem::transmute::<&[MaybeUninit<T>], &[T]>(&other.arr[other.ptr..N]);
			slice1.partial_cmp(slice2)
		}
	}
}

impl<T: Ord, const N: usize> Ord for IntoIter<T,{N}> {
	fn cmp(&self, other: &Self) -> core::cmp::Ordering {
		unsafe {
			let slice1 = core::mem::transmute::<&[MaybeUninit<T>], &[T]>(&self.arr[self.ptr..N]);
			let slice2 = core::mem::transmute::<&[MaybeUninit<T>], &[T]>(&other.arr[other.ptr..N]);
			slice1.cmp(slice2)
		}
	}
}

impl<T: core::hash::Hash, const N: usize> core::hash::Hash for IntoIter<T,{N}> {
	fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
		unsafe {
			let slice = core::mem::transmute::<&[MaybeUninit<T>], &[T]>(&self.arr[self.ptr..N]);
			slice.hash(state)
		}
	}
}

impl<T: core::fmt::Debug, const N: usize> core::fmt::Debug for IntoIter<T,{N}> {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		unsafe {
			let slice = core::mem::transmute::<&[MaybeUninit<T>], &[T]>(&self.arr[self.ptr..N]);
			slice.fmt(f)
		}
	}
}

impl<T: Default, const N: usize> Default for IntoIter<T,{N}> {
	/// Equivalent to `<[T;N]>::default().into_iter()`
	fn default() -> Self {
		new(crate::new(|_| Default::default()))
	}
}

