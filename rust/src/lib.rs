#![feature(portable_simd)]
#![cfg_attr(feature = "rustc-dep-of-std", no_std)]

pub mod handle;
pub mod protocol;

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

