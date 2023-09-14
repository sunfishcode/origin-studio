<div align="center">
  <h1><code>origin-studio</code></h1>

  <p>
    <strong>An alternative `std`-like implementation built on origin</strong>
  </p>

  <p>
    <a href="https://github.com/sunfishcode/origin-studio/actions?query=workflow%3ACI"><img src="https://github.com/sunfishcode/origin-studio/workflows/CI/badge.svg" alt="Github Actions CI Status" /></a>
    <a href="https://bytecodealliance.zulipchat.com/#narrow/stream/206238-general"><img src="https://img.shields.io/badge/zulip-join_chat-brightgreen.svg" alt="zulip chat" /></a>
    <a href="https://crates.io/crates/origin-studio"><img src="https://img.shields.io/crates/v/origin-studio.svg" alt="crates.io page" /></a>
    <a href="https://docs.rs/origin-studio"><img src="https://docs.rs/origin-studio/badge.svg" alt="docs.rs docs" /></a>
  </p>
</div>

origin-stdio is an alternative [`std`]-like implementation built on [`origin`].

At this time, it only works on Linux (x86-64, aarch64, riscv64, 32-bit x86),
requires Rust nightly, lacks full `std` compatibility, and is overall
experimental. But it supports threads and stuff.

## Quick start

In an empty directory, on Linux, with Rust nightly, run these commands:
```sh
cargo init
cargo add origin_studio
cargo add compiler_builtins --features=mem
echo 'fn main() { println!("cargo:rustc-link-arg=-nostartfiles"); }' > build.rs
sed -i '1s/^/#![no_std]\n#![no_main]\norigin_studio::no_problem!();\n\n/' src/main.rs
cargo run --quiet
```

This will produce a crate and print "Hello, world!".

Yes, you might say, I could have already done that, with just the first and
last commands. But this version uses `origin` to start and stop the program,
and [`rustix`] to do the printing.

And beyond that, origin-studio uses `origin` to start and stop threads,
[`rustix-futex-sync`] and [`lock_api`] to do locking for threads,
[`rustix-dlmalloc`] to do memory allocation, and [`unwinding`] to do stack
unwinding, so it doesn't use libc at all.

## What are those commands doing?

> cargo init

This creates a new Rust project containing a "Hello, world!" program.

> cargo add origin_studio

This adds a dependency on `origin_studio`, which is this crate.

> cargo add compiler_builtins --features=mem

This adds a dependency on `compiler_builtins`, which is a crate that provides
definitions of library functions that `rustc` and `libcore` use. The `mem`
feature enables implementations of `memcpy`, `memset`, `strlen`, and others.

> echo 'fn main() { println!("cargo:rustc-link-arg=-nostartfiles"); }' > build.rs

This creates a build.rs file that arranges for [`-nostartfiles`] to be passed
to the link command, which disables the use of libc's `crt1.o` and other startup
object files. This allows origin to define its own symbol named `_start` which
serves as the program entrypoint, and handle the entire process of starting the
program itself.

[`-nostartfiles`]: https://gcc.gnu.org/onlinedocs/gcc/Link-Options.html#index-nostartfiles

> sed -i '1s/^/#![no_std]\n#![no_main]\norigin_studio::no_problem!();\n\n/' src/main.rs

This inserts three lines to the top of src/main.rs:
 - `#![no_std]`, which disables the use of Rust's standard library
   implementation, since origin-studio provides its own implementation that
   using rustix and origin.
 - `#![no_main]`, which tells Rust to disable its code that calls the user's
   `main` function, since `origin-studio` will be handling that.
 - `origin_studio::no_problem!()` inserts code to set up a Rust panic handler,
   and optionally a global allocator (with the "alloc" feature).

> cargo run --quiet

This runs the program, which will be started by origin, prints "Hello, world!"
using origin-studio's `println!` macro, which uses origin-studio's
`std::io::stdout()` and `std::io::Write` and `rustix-futex-sync`'s `Mutex` to
do the locking, and `rustix` to do the actual I/O system call, and ends the
program, using origin.

[rustix-futex-sync]: https://github.com/sunfishcode/rustix-futex-sync

## Similar crates

Other alternative implementations of std include [steed], [tiny-std] and
[veneer].

[mustang] and [eyra] are crates that use origin to build a libc implementation
that can slide underneath existing std builds, rather than having their own std
implementations.

[relibc] also includes a Rust implementation of program and thread startup and
shutdown.

## Why?

Right now, this is a demo of how to use `origin`. If you're interested in
seeing this grow into something specific, or interested in seeing projects
which might be inspired by this, please reach out!

[`std`]: https://doc.rust-lang.org/stable/std/
[`origin`]: https://docs.rs/origin/latest/origin/
[`rustix`]: https://docs.rs/rustix/latest/rustix/
[`rustix-futex-sync`]: https://docs.rs/rustix-futex-sync/latest/rustix_futex_sync/
[`rustix-dlmalloc`]: https://docs.rs/rustix-dlmalloc/latest/rustix_dlmalloc/
[`lock_api`]: https://docs.rs/lock_api/latest/lock_api/
[`unwinding`]: https://docs.rs/unwinding/latest/unwinding/
[steed]: https://github.com/japaric/steed
[tiny-std]: https://github.com/MarcusGrass/tiny-std
[veneer]: https://crates.io/crates/veneer
[mustang]: https://github.com/sunfishcode/mustang
[eyra]: https://github.com/sunfishcode/c-ward/tree/main/eyra
[relibc]: https://gitlab.redox-os.org/redox-os/relibc/
