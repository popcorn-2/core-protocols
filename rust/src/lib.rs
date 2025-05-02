#![feature(portable_simd)]
#![no_std]

pub mod handle;
pub mod protocol;
pub use result::Result;

mod result {
	use core::marker::PhantomData;
	use core::ptr::addr_of;
	use crate::handle::Handle;

	pub unsafe trait OkTy {}

	unsafe impl OkTy for usize {}
	unsafe impl OkTy for Handle {}

	pub struct Result<T: OkTy>(isize, PhantomData<T>);

	impl<T: OkTy> Result<T> {
		fn new_ok(from: T) -> Self {
			let inner = unsafe { *addr_of!(from).cast::<isize>() };
			assert!(inner >= 0);
			Self(inner, PhantomData)
		}

		fn new(from: isize) -> Self {
			from.into()
		}

		#[track_caller]
		pub fn unwrap(self) -> T {
			if self.0 < 0 { panic!("called `Result::unwrap()` on an `Err` value: {}", -self.0) }
			unsafe { addr_of!(self.0).cast::<T>().read() }
		}
	}

	impl<T: OkTy> From<isize> for Result<T> {
		fn from(value: isize) -> Self {
			Self(value, PhantomData)
		}
	}
}

#[cfg(test)]
mod test {
	use crate::handle::Handle;
	use crate::protocol::core::io::Write;

	#[test]
	fn foo() {
		let mut handle = Handle::open().unwrap();
		handle.write(&[1,2,3,4,5]);
	}
}

