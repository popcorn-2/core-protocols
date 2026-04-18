#![feature(popcorn_protocol)]

extern crate alloc;

pub mod server {
	#![allow(warnings)]
	include!(concat!(env!("OUT_DIR"), "/server.gen.rs"));
}
