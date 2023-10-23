//! Useful synchronization primitives.

pub use alloc_crate::sync::*;
pub use core::sync::*;

pub use rustix_futex_sync::{
    Condvar, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard,
};
