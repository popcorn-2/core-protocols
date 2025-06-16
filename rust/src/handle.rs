use core::marker::PhantomData;
use crate::proto::Protocol;

#[derive(Debug)]
pub struct Handle<I> {
	raw: isize,
	_phantom: PhantomData<I>,
}

impl<I> Handle<I> {
	pub fn try_as<T: Protocol>(&self) -> Option<&Handle<T>> {
		None
	}

	pub fn as_raw(&self) -> isize { self.raw }

	pub unsafe fn from_raw(raw: isize) -> Self {
		Self {
			raw, _phantom: PhantomData
		}
	}
}

impl<I: Protocol> Handle<I> {
	pub fn new(path: &str, args: I::Ctor) -> crate::Result<Self> {
		todo!()
	}
}

impl<I> Drop for Handle<I> {
	fn drop(&mut self) {
		todo!()
	}
}
