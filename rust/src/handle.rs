use crate::protocol;

#[derive(Debug)]
#[repr(transparent)]
pub struct Handle(pub usize);

impl Handle {
	pub fn open() -> crate::Result<Self> {
		todo!()
	}
}

impl protocol::core::object::Object for Handle {
	fn as_raw_fd(&self) -> usize {
		self.0
	}
}

impl protocol::core::io::Write for Handle {}
impl protocol::core::io::Read for Handle {}
