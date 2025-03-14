#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc as alloc_crate;

// Ensure that origin is linked in.
extern crate origin;

// <https://doc.rust-lang.org/stable/std/alloc/trait.GlobalAlloc.html>
#[cfg(feature = "alloc")]
#[global_allocator]
static GLOBAL_ALLOCATOR: rustix_dlmalloc::GlobalDlmalloc = rustix_dlmalloc::GlobalDlmalloc;

// Origin calls this.
#[no_mangle]
unsafe fn origin_main(argc: i32, argv: *mut *mut u8, envp: *mut *mut u8) -> i32 {
    let _ = (argc, argv, envp);

    #[cfg(feature = "std")]
    unsafe {
        crate::init::sanitize_stdio_fds();
        crate::init::store_args(argc, argv, envp);
    }
    unsafe {
        crate::init::reset_sigpipe();
        #[cfg(feature = "stack-overflow")]
        crate::stack_overflow::init();
    }

    // Call the function expanded by the macro in the user's module to call the
    // user's `main` function.
    extern "C" {
        fn origin_studio_no_problem();
    }
    unsafe {
        origin_studio_no_problem();
    }

    rustix::runtime::EXIT_SUCCESS
}

/// 🌟
#[cfg(not(feature = "alloc"))]
#[macro_export]
macro_rules! no_problem {
    () => {
        #[doc(hidden)]
        #[no_mangle]
        extern "C" fn origin_studio_no_problem() {
            // Call the user's `main` function.
            main()
        }
    };
}

/// 🌟
#[cfg(all(feature = "alloc", not(feature = "std")))]
#[macro_export]
macro_rules! no_problem {
    () => {
        extern crate alloc;

        #[doc(hidden)]
        #[no_mangle]
        extern "C" fn origin_studio_no_problem() {
            // Call the user's `main` function.
            main()
        }
    };
}

/// 🌟
#[cfg(feature = "std")]
#[macro_export]
macro_rules! no_problem {
    () => {
        extern crate alloc;

        #[doc(hidden)]
        #[no_mangle]
        extern "C" fn origin_studio_no_problem() {
            // Call the user's `main` function.
            main()
        }

        // Provide a prelude.
        use ::alloc::{format, vec};
        use $crate::prelude::rust_2021::*;
        use $crate::{self as std, eprint, eprintln, print, println};
    };
}

mod init;
#[cfg(feature = "stack-overflow")]
mod stack_overflow;

// Provide a std-like API.

#[cfg(feature = "std")]
mod macros;

#[cfg(feature = "std")]
pub mod env;
#[cfg(feature = "std")]
#[cfg(feature = "fs")]
pub mod fs;
#[cfg(feature = "std")]
pub mod io;
#[cfg(feature = "std")]
pub mod prelude;
#[cfg(feature = "std")]
#[cfg(feature = "thread")]
pub mod sync;
#[cfg(feature = "std")]
#[cfg(feature = "thread")]
pub mod thread;

#[cfg(feature = "alloc")]
pub use alloc_crate::{
    alloc, borrow, boxed, collections, ffi, fmt, rc, slice, str, string, task, vec,
};
pub use core::*;
