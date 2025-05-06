use crate::protocol;

#[derive(Debug)]
#[repr(transparent)]
pub struct Handle(pub usize);

impl protocol::core::object::Object for Handle {
	fn as_raw_fd(&self) -> usize {
		self.0
	}
}
