mod macros;

pub mod env;
pub mod io;
pub mod prelude;
pub mod sync;
#[cfg(feature = "thread")]
pub mod thread;

pub use crate::alloc::{
    alloc, borrow, boxed, collections, ffi, fmt, rc, slice, str, string, task, vec,
};
pub use core::*;

mod sealed {
    pub trait Sealed {}
}
