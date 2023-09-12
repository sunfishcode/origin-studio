#![doc = include_str!("../README.md")]
#![allow(internal_features)]
#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(strict_provenance)]
#![deny(fuzzy_provenance_casts)]
#![deny(lossy_provenance_casts)]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc as crate_alloc;

// Ensure that origin is linked in.
extern crate origin;

// <https://doc.rust-lang.org/nomicon/panic-handler.html>
#[panic_handler]
fn panic(panic: &core::panic::PanicInfo<'_>) -> ! {
    let _ = panic;

    #[cfg(feature = "std")]
    {
        eprintln!("{}", panic);
    }
    #[cfg(all(not(feature = "std"), feature = "atomic-dbg"))]
    {
        atomic_dbg::eprintln!("{}", panic);
    }

    core::intrinsics::abort();
}

// <https://docs.rust-embedded.org/embedonomicon/smallest-no-std.html#eh_personality>
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

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
        extern crate compiler_builtins;

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
        extern crate compiler_builtins;

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
        extern crate compiler_builtins;

        #[doc(hidden)]
        #[no_mangle]
        extern "C" fn origin_studio_no_problem() {
            // Call the user's `main` function.
            main()
        }

        // Provide a prelude.
        use ::alloc::{format, vec};
        use $crate::prelude::rust_2021::*;
        use $crate::{self as std};
        use $crate::{eprint, eprintln, print, println};
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
pub mod io;
#[cfg(feature = "std")]
pub mod prelude;
#[cfg(feature = "std")]
pub mod sync;
#[cfg(feature = "std")]
#[cfg(feature = "thread")]
pub mod thread;

pub use core::*;
#[cfg(feature = "alloc")]
pub use crate_alloc::{
    alloc, borrow, boxed, collections, ffi, fmt, rc, slice, str, string, task, vec,
};

mod sealed {
    pub trait Sealed {}
}
