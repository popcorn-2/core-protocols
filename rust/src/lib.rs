#![feature(portable_simd)]
#![cfg_attr(feature = "rustc-dep-of-std", no_std)]

pub mod handle;
pub mod protocol;

#[cfg(test)]
mod test {
	use crate::create;
	use crate::handle::Handle;
	use crate::protocol::core::io::Write;

	#[test]
	fn bar() {
		let handle = create!("fs:/user/bin/bash", impl crate::protocol::core::proc::Proc);
	}
}

