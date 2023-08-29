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

Quick start:

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

## Similar crates

Other alternative implementations of std include [steed], [tiny-std] and
[veneer].

[mustang] is a crate that uses origin to build a libc implementation that can
slide underneath existing std builds, rather than having its own std
implementation.

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
