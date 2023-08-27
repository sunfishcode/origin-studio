#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]
#![feature(core_intrinsics)]

#[cfg(feature = "alloc")]
extern crate alloc;

// Ensure that origin is linked in.
extern crate origin;

// <https://doc.rust-lang.org/nomicon/panic-handler.html>
#[panic_handler]
fn panic(panic: &core::panic::PanicInfo<'_>) -> ! {
    eprintln!("{}", panic);
    core::intrinsics::abort();
}

// <https://docs.rust-embedded.org/embedonomicon/smallest-no-std.html#eh_personality>
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

// <https://doc.rust-lang.org/stable/std/alloc/trait.GlobalAlloc.html>
#[global_allocator]
static GLOBAL_ALLOCATOR: rustix_dlmalloc::GlobalDlmalloc = rustix_dlmalloc::GlobalDlmalloc;

// Origin calls this.
#[no_mangle]
extern "C" fn main(argc: i32, argv: *mut *mut u8, envp: *mut *mut u8) -> i32 {
    #[cfg(feature = "std")]
    unsafe {
        crate::init::sanitize_stdio_fds();
        crate::init::store_args(argc, argv, envp);
    }
    unsafe {
        crate::init::reset_sigpipe();
    }

    // Call the function expanded by the macro in the user's module to call the
    // user's `main` function.
    extern "C" {
        fn origin_studio_no_problem();
    }
    unsafe {
        origin_studio_no_problem();
    }

    rustix::process::EXIT_SUCCESS
}

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
        use $crate::std::{self, prelude::rust_2021::*};
        use $crate::{eprint, eprintln, print, println};
    };
}

mod init;

// Provide a std-like API.
#[cfg(feature = "std")]
pub mod std;
