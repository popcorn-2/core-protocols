pub use has_protocol::HasProtocol;

pub trait Protocol {
	type Ctor;
	const UID: &'static [u128];
}

macro_rules! protocol_tuple {
	($T:ident) => {
		protocol_tuple!(@ #[cfg_attr(doc, doc(fake_variadic))] $T);
    };
    ($($T:ident)*) => {
		protocol_tuple!(@ #[cfg_attr(doc, doc(hidden))] $($T)*);
    };
	(@ $(#[$attr:meta])* $($T:ident)*) => {
		$(#[$attr])*
	    impl<$($T: Protocol),*> Protocol for ($($T),*,) {
			type Ctor = ($($T),*,);
			const UID: &'static [u128] = &[
				$(<$T as Protocol>::UID[0]),*
			];
		}
    };
}

protocol_tuple!(T);
protocol_tuple!(T U);
protocol_tuple!(T U V);
protocol_tuple!(T U V W);
protocol_tuple!(T U V W X);

mod has_protocol {
	use super::Protocol;

	#[marker]
	pub trait HasProtocol<T: Protocol> {}

	impl<T: Protocol> HasProtocol<T> for T {}
	#[cfg_attr(doc, doc(fake_variadic))]
	impl<T: Protocol> HasProtocol<T> for (T,) {}
	#[doc(hidden)]
	impl<T: Protocol, U> HasProtocol<T> for (T, U) {}
	#[doc(hidden)]
	impl<T: Protocol, U> HasProtocol<T> for (U, T) {}
	#[doc(hidden)]
	impl<T: Protocol, U, V> HasProtocol<T> for (T, U, V) {}
	#[doc(hidden)]
	impl<T: Protocol, U, V> HasProtocol<T> for (U, T, V) {}
	#[doc(hidden)]
	impl<T: Protocol, U, V> HasProtocol<T> for (U, V, T) {}
	#[doc(hidden)]
	impl<T: Protocol, U, V, W> HasProtocol<T> for (T, U, V, W) {}
	#[doc(hidden)]
	impl<T: Protocol, U, V, W> HasProtocol<T> for (U, T, V, W) {}
	#[doc(hidden)]
	impl<T: Protocol, U, V, W> HasProtocol<T> for (U, V, T, W) {}
	#[doc(hidden)]
	impl<T: Protocol, U, V, W> HasProtocol<T> for (U, V, W, T) {}
	#[doc(hidden)]
	impl<T: Protocol, U, V, W, X> HasProtocol<T> for (T, U, V, W, X) {}
	#[doc(hidden)]
	impl<T: Protocol, U, V, W, X> HasProtocol<T> for (U, T, V, W, X) {}
	#[doc(hidden)]
	impl<T: Protocol, U, V, W, X> HasProtocol<T> for (U, V, T, W, X) {}
	#[doc(hidden)]
	impl<T: Protocol, U, V, W, X> HasProtocol<T> for (U, V, W, T, X) {}
	#[doc(hidden)]
	impl<T: Protocol, U, V, W, X> HasProtocol<T> for (U, V, W, X, T) {}
}

macro_rules! protocol {
	() => {};
    (pub protocol $name:ident = $uid:literal {
	    ctor$(<$($ctor_lifetime:lifetime),*>)? => {
		    $($ctor_arg:ident : $ctor_ty:ty),* $(,)?
	    }

	    $(fn $fn_name:ident(&self $(, $fn_arg:ident: $fn_ty:ty)* $(,)?) $(-> $fn_ret:ty)? $f:block);*
    } $($rest:tt)*) => {
	    pub struct $name $(<$($ctor_lifetime),*>)? {
		    $(pub $ctor_arg : $ctor_ty),*
	    }

	    impl $(<$($ctor_lifetime),*>)? $crate::proto::Protocol for $name $(<$($ctor_lifetime),*>)? {
		    type Ctor = Self;
		    const UID: &'static [u128] = &[$uid];
	    }

	    pub trait ${concat($name, Tr)} {

	    }

	    impl<$($($ctor_lifetime),*,)? I: $crate::proto::HasProtocol<$name $(<$($ctor_lifetime),*>)?>> ${concat($name, Tr)} for $crate::handle::RawHandle<I> {}

	    protocol!($($rest)*);
    };
}

pub mod core {
	pub mod fs {
		protocol! {
			pub protocol File = 1 {
				ctor => {
					create: usize,
					append: bool,
					truncate: bool,
				}
			}
		}
	}

	pub mod io {
		protocol! {
			pub protocol Read = 2 {
				ctor => {}
			}

			pub protocol Write = 3 {
				ctor => {}
			}

			pub protocol Seek = 4 {
				ctor => {}
			}

			pub protocol Terminal = 5 {
				ctor => {}
			}
		}
	}

	pub mod proc {
		protocol! {
			pub protocol ProcessBuilder = 6 {
				ctor<'a> => {
					name: &'a str,
				}
			}

			pub protocol Process = 7 {
				ctor => {}
			}
		}
	}
}