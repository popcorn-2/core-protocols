use core::fmt::{Debug, Formatter};
use core::marker::PhantomData;
use crate::proto::{HasProtocol, Protocol};

#[derive(Debug, Copy, Clone)]
pub struct RawHandle(pub isize);

pub struct Handle<I> {
	handle: RawHandle,
	_phantom: PhantomData<I>,
}

impl<I: Protocol> HasProtocol<I> for Handle<I> {}

impl<I> Debug for Handle<I> {
	fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
		write!(f, "Handle::<{}>({})", core::any::type_name::<I>(), self.handle.0)
	}
}

impl RawHandle {
	pub fn destroy(&self) {
		todo!()
	}

	pub fn has_protocol<T: Protocol>(&self) -> bool {
		false
	}
}

impl RawHandle {
	pub fn new<I: Protocol>(path: &str, args: I::Ctor) -> crate::Result<Self> {
		todo!()
	}
}

impl<I: Protocol> Handle<I> {
	pub fn new(path: &str, args: I::Ctor) -> crate::Result<Self> {
		Ok(Self {
			handle: RawHandle::new::<I>(path, args)?,
			_phantom: PhantomData,
		})
	}
}

impl<I> Handle<I> {
	pub fn try_as<T: Protocol>(&self) -> Option<&Handle<T>> {
		if self.handle.has_protocol::<T>() { Some(unsafe { core::mem::transmute(self) }) }
		else { None }
	}
}

impl<I> Drop for Handle<I> {
	fn drop(&mut self) {
		self.handle.destroy();
	}
}

pub trait FromRawHandle {
	unsafe fn from_raw_handle(handle: RawHandle) -> Self;
}

impl<I> FromRawHandle for Handle<I> {
	unsafe fn from_raw_handle(handle: RawHandle) -> Self {
		Self {
			handle, _phantom: PhantomData
		}
	}
}

pub trait AsRawHandle {
	fn as_raw_handle(&self) -> RawHandle;
}

impl<I> AsRawHandle for Handle<I> {
	fn as_raw_handle(&self) -> RawHandle {
		self.handle
	}
}

#[cfg(test)]
mod test {
	use std::marker::PhantomData;
	use crate::handle::{Handle, RawHandle};
	use crate::proto::core::io::Read;

	trait Foo {
		fn foo(&self);
	}
	impl<H: crate::handle::AsRawHandle + crate::proto::HasProtocol<Read>> Foo for H {
		fn foo(&self) {
			self.as_raw_handle();
		}
	}

	fn foo() {
		let handle = Handle::<Read>::new("foo", Read {}).unwrap();
		handle.foo();
	}
}
