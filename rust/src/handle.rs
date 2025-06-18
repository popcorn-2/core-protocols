use core::ops::Deref;
use core::marker::PhantomData;
use crate::proto::Protocol;

#[derive(Debug)]
pub struct RawHandle<I> {
	raw: isize,
	_phantom: PhantomData<I>,
}

impl<I> Clone for RawHandle<I> {
	fn clone(&self) -> Self {
		Self { raw: self.raw, _phantom: PhantomData }
	}
}

impl<I> Copy for RawHandle<I> {}

#[derive(Debug)]
pub struct Handle<I> {
	handle: RawHandle<I>,
}

impl<I> RawHandle<I> {
	pub fn destroy(&self) {
		todo!()
	}

	pub fn has_protocol<T: Protocol>(&self) -> bool {
		false
	}

	pub fn as_raw(&self) -> isize { self.raw }

	pub fn from_raw(raw: isize) -> Self {
		Self {
			raw, _phantom: PhantomData
		}
	}
}

impl<I: Protocol> RawHandle<I> {
	pub fn new(path: &str, args: I::Ctor) -> crate::Result<Self> {
		todo!()
	}
}

impl<I: Protocol> Handle<I> {
	pub fn new(path: &str, args: I::Ctor) -> crate::Result<Self> {
		todo!()
	}
}

impl<I> Handle<I> {
	pub fn try_as<T: Protocol>(&self) -> Option<&Handle<T>> {
		if self.has_protocol::<T>() { Some(unsafe { core::mem::transmute(self) }) }
		else { None }
	}
}

impl<I> Deref for Handle<I> {
	type Target = RawHandle<I>;

	fn deref(&self) -> &Self::Target {
		&self.handle
	}
}

impl<I> Drop for Handle<I> {
	fn drop(&mut self) {
		self.handle.destroy();
	}
}

pub trait FromRawHandle<I> {
	unsafe fn from_raw_handle(handle: RawHandle<I>) -> Self;
}

impl<I> FromRawHandle<I> for Handle<I> {
	unsafe fn from_raw_handle(handle: RawHandle<I>) -> Self {
		Self {
			handle
		}
	}
}
