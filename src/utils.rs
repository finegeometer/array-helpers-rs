use core::mem::MaybeUninit;

/// This is the same thing as core::mem::transmute, but without the check that the types are the same size.
/// This is necessary because core::mem::transmute refuses to transmute between dependently-sized types. 
unsafe fn unsafe_transmute<A, B>(a: A) -> B {
	let ptr = &a as *const A as *const B;
	core::mem::forget(a);
	core::ptr::read(ptr)
}

/// Reinterpret a possibly-initialized array of things as an array of possibly-initialized things.
pub fn push_maybe_uninit<T, const N: usize>(arr: MaybeUninit<[T;N]>) -> [MaybeUninit<T>;N] {
	unsafe { unsafe_transmute(arr) }
}

/// Reinterpret an array of possibly-initialized things as a possibly-initialized array of things.
pub fn pull_maybe_uninit<T, const N: usize>(arr: [MaybeUninit<T>;N]) -> MaybeUninit<[T;N]> {
	unsafe { unsafe_transmute(arr) }
}

/// Extract a value from an array, leaving it uninitialized. Undefined Behavior if it is already uninitialized.
pub unsafe fn extract<T, const N: usize>(arr: &mut [MaybeUninit<T>;N], i: usize) -> T {
	core::mem::replace(&mut arr[i], MaybeUninit::uninit()).assume_init()
}
