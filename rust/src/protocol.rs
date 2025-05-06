#[cfg(not(feature = "rustc-dep-of-std"))] type E<T> = std::io::Result<T>;
#[cfg(feature = "rustc-dep-of-std")] type E<T> = Result<T, isize>;

macro_rules! method {
    ($name:ident @ $id:literal ($($arg_name: ident : $arg_ty: ty),* $(,)?) => ($b:expr, $c:expr, $d:expr)) => {
	    fn $name(&mut self, $($arg_name : $arg_ty),*) -> $crate::protocol::E<isize> {
		    Self::__syscall(
					Self::UID | (($id as u128) << 96),
					self.as_raw_fd(),
					$b as _,
					$c as _,
					$d as _,
		    )
	    }
    };
}

pub mod core {
	pub mod server {
		use core::mem::MaybeUninit;

		#[repr(C)]
		pub struct Packet {

		}

		pub trait Sync: super::object::Object {
			const UID: u128 = 7;

			method!(get@0(data: &mut [MaybeUninit<Packet>]) => (0, data.as_ptr(), data.len()));
		}
		impl Sync for crate::handle::Handle {}
	}

	pub mod proc {
		pub trait Proc: super::object::Object {
			const UID: u128 = 5;

			method!(exit@0() => (0,0,0));
			method!(debug@1(s: &str) => (0, s.as_ptr(), s.len()));
			method!(unstable_anon_alloc@2(size: usize) => (size, 0, 0));
			method!(unstable_anon_dealloc@3(ptr: *mut u8, size: usize) => (ptr, size, 0));
		}
		impl Proc for crate::handle::Handle {}

		pub trait Thread: super::object::Object {
			const UID: u128 = 7;

			method!(set_tcb@0(tcb: *mut u8) => (tcb, 0, 0));
			method!(
				exec@1(
					f: extern "C" fn(*mut core::ffi::c_void) -> *mut core::ffi::c_void,
					arg: *mut core::ffi::c_void,
					stack_top: *mut u8,
				) => (f, arg, stack_top)
			);
			method!(join@2() => (0, 0, 0));
		}
		impl Thread for crate::handle::Handle {}
	}

	pub mod io {
		pub trait Write: super::object::Object {
			const UID: u128 = 2;

			method!(write@0(data: &[u8]) => (0, data.as_ptr(), data.len()));
		}
		impl Write for crate::handle::Handle {}

		pub trait Read: super::object::Object {
			const UID: u128 = 3;

			method!(read@0(data: &mut [u8]) => (0, data.as_mut_ptr(), data.len()));
		}
		impl Read for crate::handle::Handle {}
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
			fn __syscall(proto_method: u128, a: usize, b: usize, c: usize, d: usize) -> crate::protocol::E<isize> {
				let num = u32x4::from_array([
					proto_method as _,
					(proto_method >> 32) as _,
					(proto_method >> 64) as _,
					(proto_method >> 96) as _,
				]);

				let ret: isize;
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

				#[cfg(feature = "rustc-dep-of-std")] {
					if ret < 0 { Err(-ret) }
					else { Ok(ret) }
				}

				#[cfg(not(feature = "rustc-dep-of-std"))] {
					if ret < 0 { Err(std::io::Error::from_raw_os_error(-ret)) }
					else { Ok(ret) }
				}
			}

			fn as_raw_fd(&self) -> usize;
		}
	}
}
#[cfg(not(feature = "rustc-dep-of-std"))]
#[cfg(target_os = "popcorn")]
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

		fn shim() ->  Result<impl $tr $(+ $tr2)*, ::std::io::Error> {
			let path: &str = $path;
			let res = <$crate::handle::Handle as $crate::protocol::core::object::Object>::__syscall(
				0,
				path.as_ptr() as usize,
				path.len(),
				0,
				0,
			);

			res.0.map(|h| $crate::handle::Handle)
			     .map_err(|e| ::std::io::Error::from_raw_os_error)
		}

	    shim()
    }};
}

#[cfg(feature = "rustc-dep-of-std")]
#[cfg(target_os = "popcorn")]
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

		fn shim() -> Result<impl $tr $(+ $tr2)*, crate::io::Error> {
			let path: &str = $path;
			let res = <$crate::handle::Handle as $crate::protocol::core::object::Object>::__syscall(
				0,
				path.as_ptr() as usize,
				path.len(),
				0,
				0,
			);

			res.0.map(|h| $crate::handle::Handle)
			     .map_err(|e| crate::io::Error::from_raw_os_error)
		}

	    shim()
    }};
}
