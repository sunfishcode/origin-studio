use crate::std::io;
use alloc::boxed::Box;
use core::num::NonZeroUsize;

// Rust does't need the OS tids, it just needs unique ids, so we just use the
// raw `Thread` value casted to `usize`.
pub struct ThreadId(usize);

pub struct Thread(origin::thread::Thread);

impl Thread {
    pub fn id(&self) -> ThreadId {
        ThreadId(self.0.to_raw() as usize)
    }
}

pub struct JoinHandle(Thread);

impl JoinHandle {
    pub fn join(self) -> io::Result<()> {
        unsafe {
            origin::thread::join_thread(self.0 .0);
        }

        // Don't call drop, which would detach the thread we just joined.
        core::mem::forget(self);

        Ok(())
    }
}

impl Drop for JoinHandle {
    fn drop(&mut self) {
        unsafe {
            origin::thread::detach_thread(self.0 .0);
        }
    }
}

pub fn spawn<F>(f: F) -> JoinHandle
where
    F: FnOnce() + Send + 'static,
{
    let thread = origin::thread::create_thread(
        Box::new(|| {
            unsafe {
                let _handler = crate::stack_overflow::Handler::new();
            }

            f();
            None
        }),
        origin::thread::default_stack_size(),
        origin::thread::default_guard_size(),
    )
    .unwrap();

    JoinHandle(Thread(thread))
}

pub fn current() -> Thread {
    Thread(origin::thread::current_thread())
}

pub(crate) struct GetThreadId;

unsafe impl rustix_futex_sync::lock_api::GetThreadId for GetThreadId {
    const INIT: Self = Self;

    fn nonzero_thread_id(&self) -> NonZeroUsize {
        // TODO: Use `origin::thread::currrent_thread().addr()` once that's stable.
        NonZeroUsize::new(origin::thread::current_thread().to_raw_non_null().as_ptr() as usize).unwrap()
    }
}

pub(crate) type ReentrantMutex<T> = rustix_futex_sync::ReentrantMutex<GetThreadId, T>;
pub(crate) type ReentrantMutexGuard<'a, T> =
    rustix_futex_sync::ReentrantMutexGuard<'a, GetThreadId, T>;
