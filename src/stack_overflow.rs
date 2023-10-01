//! The following is derived from Rust's
//! library/std/src/sys/unix/stack_overflow.rs at revision
//! 3e35b39d9dbfcd937c6b9163a3514d6a4775c198.

macro_rules! rtprintpanic {
    ($($t:tt)*) => {
        #[cfg(feature = "std")]
        let _ = $crate::io::Write::write_fmt(&mut $crate::io::stderr(), format_args!($($t)*));
    }
}

macro_rules! rtabort {
    ($($t:tt)*) => {
        {
            rtprintpanic!("fatal runtime error: {}\n", format_args!($($t)*));
            core::intrinsics::abort();
        }
    }
}

use core::ffi::c_void;

pub(crate) struct Handler {
    data: *mut c_void,
}

impl Handler {
    pub(crate) unsafe fn new() -> Handler {
        make_handler()
    }

    fn null() -> Handler {
        Handler {
            data: core::ptr::null_mut(),
        }
    }
}

impl Drop for Handler {
    fn drop(&mut self) {
        unsafe {
            drop_handler(self.data);
        }
    }
}

use core::{mem, ptr};

use origin::signal::{
    sigaction, SigDfl, Sigaction, Siginfo, Signal, SA_ONSTACK, SA_SIGINFO, SIGSTKSZ, SS_DISABLE,
};
use rustix::mm::{mmap_anonymous, munmap, MapFlags, ProtFlags};
use rustix::runtime::sigaltstack;

use core::sync::atomic::{AtomicBool, AtomicPtr, Ordering};
use rustix::param::page_size;

// Signal handler for the SIGSEGV and SIGBUS handlers. We've got guard pages
// (unmapped pages) at the end of every thread's stack, so if a thread ends
// up running into the guard page it'll trigger this handler. We want to
// detect these cases and print out a helpful error saying that the stack
// has overflowed. All other signals, however, should go back to what they
// were originally supposed to do.
//
// This handler currently exists purely to print an informative message
// whenever a thread overflows its stack. We then abort to exit and
// indicate a crash, but to avoid a misleading SIGSEGV that might lead
// users to believe that unsafe code has accessed an invalid pointer; the
// SIGSEGV encountered when overflowing the stack is expected and
// well-defined.
//
// If this is not a stack overflow, the handler un-registers itself and
// then returns (to allow the original signal to be delivered again).
// Returning from this kind of signal handler is technically not defined
// to work when reading the POSIX spec strictly, but in practice it turns
// out many large systems and all implementations allow returning from a
// signal handler to work. For a more detailed explanation see the
// comments on #26458.
unsafe extern "C" fn signal_handler(signum: Signal, info: *mut Siginfo, _data: *mut c_void) {
    let (stack_addr, _stack_size, guard_size) =
        origin::thread::thread_stack(origin::thread::current_thread());
    let guard_end = stack_addr.addr();
    let guard_start = guard_end - guard_size;

    let addr = (*info)
        .__bindgen_anon_1
        .__bindgen_anon_1
        ._sifields
        ._sigfault
        ._addr
        .addr();

    // If the faulting address is within the guard page, then we print a
    // message saying so and abort.
    if guard_start <= addr && addr < guard_end {
        rtprintpanic!(
            "\nthread '{}' has overflowed its stack\n",
            rustix::thread::name()
                .map(|c_str| alloc_crate::format!("{:?}", c_str))
                .unwrap_or("<unknown>".into())
        );
        rtabort!("stack overflow");
    } else {
        // Unregister ourselves by reverting back to the default behavior.
        let mut action: Sigaction = mem::zeroed();
        action.sa_handler_kernel = SigDfl;
        let _ = sigaction(signum, Some(action));

        // See comment above for why this function returns.
    }
}

static MAIN_ALTSTACK: AtomicPtr<c_void> = AtomicPtr::new(ptr::null_mut());
static NEED_ALTSTACK: AtomicBool = AtomicBool::new(false);

pub unsafe fn init() {
    for &signal in &[Signal::Segv, Signal::Bus] {
        let mut action = sigaction(signal, None).unwrap();
        // Configure our signal handler if one is not already set.
        if action.sa_handler_kernel == SigDfl {
            action.sa_flags = SA_SIGINFO | SA_ONSTACK;
            action.sa_handler_kernel = Some(mem::transmute(
                signal_handler as unsafe extern "C" fn(_, _, _),
            ));
            let _ = sigaction(signal, Some(action));
            NEED_ALTSTACK.store(true, Ordering::Relaxed);
        }
    }

    let handler = make_handler();
    MAIN_ALTSTACK.store(handler.data, Ordering::Relaxed);
    mem::forget(handler);
}

// Calling the cleanup function isn't necessary.
/*
pub unsafe fn cleanup() {
    drop_handler(MAIN_ALTSTACK.load(Ordering::Relaxed));
}
*/

unsafe fn get_stackp() -> *mut c_void {
    // OpenBSD requires this flag for stack mapping
    // otherwise the said mapping will fail as a no-op on most systems
    // and has a different meaning on FreeBSD
    #[cfg(any(target_os = "openbsd", target_os = "netbsd", target_os = "linux"))]
    let flags = MapFlags::PRIVATE | MapFlags::STACK;
    #[cfg(not(any(target_os = "openbsd", target_os = "netbsd", target_os = "linux")))]
    let flags = MapFlags::PRIVATE;
    let stackp = match mmap_anonymous(
        ptr::null_mut(),
        SIGSTKSZ + page_size(),
        ProtFlags::READ | ProtFlags::WRITE,
        flags,
    ) {
        Ok(stackp) => stackp,
        Err(err) => panic!("failed to allocate an alternative stack: {}", err),
    };
    match rustix::mm::mprotect(stackp, page_size(), rustix::mm::MprotectFlags::empty()) {
        Ok(guard_result) => guard_result,
        Err(err) => panic!("failed to set up alternative stack guard page: {}", err),
    };
    stackp.add(page_size())
}

unsafe fn get_stack() -> rustix::runtime::Stack {
    rustix::runtime::Stack {
        ss_sp: get_stackp(),
        ss_flags: 0,
        ss_size: SIGSTKSZ as _,
    }
}

unsafe fn make_handler() -> Handler {
    if !NEED_ALTSTACK.load(Ordering::Relaxed) {
        return Handler::null();
    }
    let mut stack = sigaltstack(None).unwrap();
    // Configure alternate signal stack, if one is not already set.
    if stack.ss_flags & SS_DISABLE != 0 {
        stack = get_stack();
        let _ = sigaltstack(Some(stack)).unwrap();
        Handler {
            data: stack.ss_sp as *mut c_void,
        }
    } else {
        Handler::null()
    }
}

unsafe fn drop_handler(data: *mut c_void) {
    if !data.is_null() {
        let stack = rustix::runtime::Stack {
            ss_sp: ptr::null_mut(),
            ss_flags: SS_DISABLE,
            // Workaround for bug in macOS implementation of sigaltstack
            // UNIX2003 which returns ENOMEM when disabling a stack while
            // passing ss_size smaller than MINSIGSTKSZ. According to POSIX
            // both ss_sp and ss_size should be ignored in this case.
            ss_size: SIGSTKSZ as _,
        };
        let _ = sigaltstack(Some(stack));
        // We know from `get_stackp` that the alternate stack we installed is part of a
        // mapping that started one page earlier, so walk back a page and unmap
        // from there.
        let _ = munmap(data.sub(page_size()), SIGSTKSZ + page_size());
    }
}
