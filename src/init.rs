#[cfg(feature = "std")]
pub(crate) fn sanitize_stdio_fds() {
    use rustix::cstr;
    use rustix::fd::BorrowedFd;
    use rustix::fs::{open, Mode, OFlags};
    use rustix::io::{fcntl_getfd, Errno};

    for raw_fd in 0..3 {
        let fd = unsafe { BorrowedFd::borrow_raw(raw_fd) };
        if let Err(Errno::BADF) = fcntl_getfd(fd) {
            let _ = open(cstr!("/dev/null"), OFlags::RDWR, Mode::empty()).unwrap();
        }
    }
}

#[cfg(feature = "std")]
pub(crate) unsafe fn store_args(argc: i32, argv: *mut *mut u8, envp: *mut *mut u8) {
    crate::env::MAIN_ARGS = crate::env::MainArgs { argc, argv, envp };
}

pub(crate) unsafe fn reset_sigpipe() {
    use core::mem::zeroed;
    use origin::signal::{sig_ign, sigaction, Sigaction, SigactionFlags, Signal};

    let mut action = zeroed::<Sigaction>();
    action.sa_handler_kernel = sig_ign();
    action.sa_flags = SigactionFlags::RESTART;
    sigaction(Signal::PIPE, Some(action)).unwrap();
}
