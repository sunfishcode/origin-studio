use crate::std::io;
use core::num::NonZeroUsize;
use alloc::boxed::Box;

// FIXME: When bytecodealliance/rustix#796 lands, switch to rustix::thread.
pub struct ThreadId(rustix::process::Pid);

pub struct Thread(origin::Thread);

impl Thread {
    pub fn id(&self) -> ThreadId {
        //FIXME: origin::thread_id(self.0)
        todo!()
    }
}

pub(crate) struct GetThreadId;

unsafe impl rustix_futex_sync::lock_api::GetThreadId for GetThreadId {
    const INIT: Self = Self;

    fn nonzero_thread_id(&self) -> NonZeroUsize {
        origin::current_thread_id().as_raw_nonzero().try_into().unwrap()
    }
}

pub struct JoinHandle(Thread);

impl JoinHandle {
    pub fn join(self) -> io::Result<()> {
        unsafe {
            origin::join_thread(self.0 .0);
        }

        // Don't call drop, which would detach the thread we just joined.
        core::mem::forget(self);

        Ok(())
    }
}

impl Drop for JoinHandle {
    fn drop(&mut self) {
        unsafe {
            origin::detach_thread(self.0 .0);
        }
    }
}

pub fn spawn<F>(f: F) -> JoinHandle
where
    F: FnOnce() -> () + Send + 'static,
{
    let thread = origin::create_thread(
        Box::new(|| {
            f();
            None
        }),
        origin::default_stack_size(),
        origin::default_guard_size(),
    )
    .unwrap();

    JoinHandle(Thread(thread))
}

pub fn current() -> Thread {
    Thread(origin::current_thread())
}
