use crate::protocol;

#[derive(Debug)]
#[repr(transparent)]
pub struct Handle(pub isize);

impl protocol::core::object::Object for Handle {
	fn as_raw_fd(&self) -> isize {
		self.0
	}
}
