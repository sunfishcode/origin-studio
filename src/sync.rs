//! Useful synchronization primitives.

pub use core::sync::*;
pub use crate_alloc::sync::*;

pub use rustix_futex_sync::{
    Condvar, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard,
};
