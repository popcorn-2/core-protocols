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

		pub trait Object {
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