#[cfg(feature = "std")]
pub(crate) fn sanitize_stdio_fds() {
    use rustix::cstr;
    use rustix::fd::BorrowedFd;
    use rustix::fs::{fcntl_getfd, open, Mode, OFlags};
    use rustix::io::Errno;

    for raw_fd in 0..3 {
        let fd = unsafe { BorrowedFd::borrow_raw(raw_fd) };
        if let Err(Errno::BADF) = fcntl_getfd(fd) {
            let _ = open(cstr!("/dev/null"), OFlags::RDWR, Mode::empty()).unwrap();
        }
    }
}

#[cfg(feature = "std")]
pub(crate) unsafe fn store_args(argc: i32, argv: *mut *mut u8, envp: *mut *mut u8) {
    crate::std::env::MAIN_ARGS = crate::std::env::MainArgs { argc, argv, envp };
}

pub(crate) unsafe fn reset_sigpipe() {
    use core::mem::zeroed;
    use origin::signal::{sig_ign, sigaction, Sigaction, Signal, SA_RESTART};

    let mut action = zeroed::<Sigaction>();
    action.sa_handler_kernel = sig_ign();
    action.sa_flags = SA_RESTART;
    sigaction(Signal::Pipe, Some(action)).unwrap();
}
