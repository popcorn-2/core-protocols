pub mod core {
	pub mod io {
		pub trait Write: super::object::Object {
			const UID: u128 = 2;
			const WRITE: u128 = 0;
			const PUT: u128 = 1<<96;

			fn write(&mut self, data: &[u8]) -> crate::Result<usize> {
				Self::__syscall(
					Self::UID | Self::WRITE,
					self.as_raw_fd(),
					0,
					data.as_ptr() as _,
					data.len(),
				).into()
			}
		}

		pub trait Read: super::object::Object {
			const UID: u128 = 3;
			const READ: u128 = 0;
			const GET: u128 = 1<<96;

			fn read(&mut self, data: &mut [u8]) -> crate::Result<usize> {
				Self::__syscall(
					Self::UID | Self::READ,
					self.as_raw_fd(),
					0,
					data.as_mut_ptr() as _,
					data.len(),
				).into()
			}
		}
	}

	pub mod object {
		use core::simd::u32x4;

		mod private {
			pub trait Sealed {}

			impl Sealed for crate::handle::Handle {}
		}

		#[diagnostic::on_unimplemented(
			message = "cannot pass trait to `create!()` which isn't a protocol"
		)]
		pub trait Object: private::Sealed {
			fn __syscall(proto_method: u128, a: usize, b: usize, c: usize, d: usize) -> isize {
				let num = u32x4::from_array([
					proto_method as _,
					(proto_method >> 32) as _,
					(proto_method >> 64) as _,
					(proto_method >> 96) as _,
				]);

				let ret;
				unsafe {
					#[cfg(target_arch = "x86_64")]
					core::arch::asm!(
						"xchg r12, rbx",
						"syscall",
						"xchg r12, rbx",
						inout("xmm0") num => _,
						inout("rax") a => ret,
						inout("rdi") b => _,
						inout("rsi") c => _,
						inout("rdx") d => _,
						out("rcx") _,
						out("r11") _,
						out("r12") _,
					);
				}
				ret
			}

			fn as_raw_fd(&self) -> usize;
		}
	}
}

#[cfg(not(feature = "rustc-dep-of-std"))]
#[macro_export]
macro_rules! create {
    ($path:literal, impl $tr:path $(| $tr2:path)*) => {{
	    struct _Test<T: $crate::protocol::core::object::Object>(::core::marker::PhantomData<T>);
	    {
		    struct _Test2<T: $tr>(_Test<T>);
	    }
	    $({
		    struct _Test2<T: $tr2>(_Test<T>);
	    })*

		fn shim() -> Result<impl $tr $(+ $tr2)*, ::std::io::Error> {
			let path: &str = $path;
			let res = <$crate::handle::Handle as $crate::protocol::core::object::Object>::__syscall(
				0,
				path.as_ptr() as usize,
				path.len(),
				0,
				0,
			);

		    if res >= 0 {
				Ok($crate::handle::Handle(res as usize))
			} else {
				Err(::std::io::Error::from_raw_os_error(-res))
			}
		}

	    shim()
    }};
}

