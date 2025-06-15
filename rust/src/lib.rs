#![feature(marker_trait_attr)]
#![feature(macro_metavar_expr_concat)]
#![cfg_attr(doc, feature(rustdoc_internals))]
#![cfg_attr(any(feature = "rustc-dep-of-std", not(feature = "std")), no_std)]

pub mod handle;
pub mod proto;

#[derive(Debug)]
#[repr(isize)]
pub enum SyscallError {
	Unimplemented = 1,
	InvalidUtf8 = 2,
	Overflow = 3,
	InvalidPointer = 4,
	InvalidArg = 5,
	NameInUse = 6,
	BadServer = 7,
	BadHandle = 8,
	ServerDead = 9,
	AllocationFailure = 10,
}

pub type Result<T> = core::result::Result<T, SyscallError>;
