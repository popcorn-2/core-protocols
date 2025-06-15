use std::ffi::OsStr;
use core::marker::PhantomData;
use crate::proto::Protocol;

pub struct Handle<I> {
	raw: isize,
	_phantom: PhantomData<I>,
}

impl<I: Protocol> Handle<I> {
	pub fn new(path: impl AsRef<OsStr>, args: I::Ctor) -> crate::Result<Self> {
		todo!()
	}

	pub fn try_as<T: Protocol>(&self) -> Option<&Handle<T>> {
		None
	}
}

impl<I> Drop for Handle<I> {
	fn drop(&mut self) {
		todo!()
	}
}
