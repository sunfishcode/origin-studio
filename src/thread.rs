//! Native threads.

use crate::boxed::Box;
use crate::io;
use core::mem::forget;
use core::num::NonZeroUsize;
use core::ptr::{null_mut, NonNull};

// Rust does't need the OS tids, it just needs unique ids, so we just use the
// raw `Thread` value casted to `usize`.
#[allow(dead_code)] // TODO: obviate this
pub struct ThreadId(usize);

pub struct Thread(origin::thread::Thread);

impl Thread {
    pub fn id(&self) -> ThreadId {
        ThreadId(self.0.to_raw().addr())
    }
}

pub struct JoinHandle(Thread);

impl JoinHandle {
    pub fn join(self) -> io::Result<()> {
        unsafe {
            origin::thread::join(self.0 .0);
        }

        // Don't call drop, which would detach the thread we just joined.
        forget(self);

        Ok(())
    }
}

impl Drop for JoinHandle {
    fn drop(&mut self) {
        unsafe {
            origin::thread::detach(self.0 .0);
        }
    }
}

pub fn spawn<F>(f: F) -> JoinHandle
where
    F: FnOnce() + Send + 'static,
{
    // Pack up the closure.
    let boxed: Box<dyn FnOnce() + Send + 'static> = Box::new(move || {
        #[cfg(feature = "stack-overflow")]
        let _handler = unsafe { crate::stack_overflow::Handler::new() };

        f()
    });

    // We could avoid double boxing by enabling the unstable `ptr_metadata`
    // feature, using `.to_raw_parts()` on the box pointer, though it does
    // also require transmuting the metadata into `*mut c_void` and back.
    /*
    let raw: *mut (dyn FnOnce() + Send + 'static) = Box::into_raw(boxed);
    let (callee, metadata) = raw.to_raw_parts();
    let args = [
        NonNull::new(callee as _),
        NonNull::new(unsafe { transmute(metadata) }),
    ];
    */
    let boxed = Box::new(boxed);
    let raw: *mut Box<dyn FnOnce() + Send + 'static> = Box::into_raw(boxed);
    let args = [NonNull::new(raw.cast())];

    let thread = unsafe {
        let r = origin::thread::create(
            move |args| {
                // Unpack and call.
                /*
                let (callee, metadata) = (args[0], args[1]);
                let raw: *mut (dyn FnOnce() + Send + 'static) =
                    ptr::from_raw_parts_mut(transmute(callee), transmute(metadata));
                let boxed = Box::from_raw(raw);
                boxed();
                */
                let raw: *mut Box<dyn FnOnce() + Send + 'static> = match args[0] {
                    Some(raw) => raw.as_ptr().cast(),
                    None => null_mut(),
                };
                let boxed: Box<Box<dyn FnOnce() + Send + 'static>> = Box::from_raw(raw);
                (*boxed)();

                None
            },
            &args,
            origin::thread::default_stack_size(),
            origin::thread::default_guard_size(),
        );
        r.unwrap()
    };

    JoinHandle(Thread(thread))
}

pub fn current() -> Thread {
    Thread(origin::thread::current())
}

pub(crate) struct GetThreadId;

unsafe impl rustix_futex_sync::lock_api::GetThreadId for GetThreadId {
    const INIT: Self = Self;

    fn nonzero_thread_id(&self) -> NonZeroUsize {
        origin::thread::current().to_raw_non_null().addr()
    }
}

pub(crate) type ReentrantMutex<T> = rustix_futex_sync::ReentrantMutex<GetThreadId, T>;
pub(crate) type ReentrantMutexGuard<'a, T> =
    rustix_futex_sync::ReentrantMutexGuard<'a, GetThreadId, T>;
