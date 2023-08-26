pub use crate::alloc::sync::*;
pub use core::sync::*;

pub use rustix_futex_sync::{
    Condvar, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard,
};
