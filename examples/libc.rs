#![no_std]
#![no_main]
origin_studio::no_problem!();

fn main() {
    // Call functions declared in the `libc` crate, which will be resolved by
    // c-scape.
    unsafe {
        // Call functions declared in the `libc` crate, which will be resolved by
        // c-scape. c-scape doesn't have `printf`, so we do it by hand.
        let message = b"Hello, world!\n";
        let mut remaining = &message[..];
        while !remaining.is_empty() {
            match libc::write(1, message.as_ptr().cast(), message.len()) {
                -1 => match errno::errno().0 {
                    libc::EINTR => continue,
                    _ => panic!(),
                },
                n => remaining = &remaining[n as usize..],
            }
        }
        libc::exit(0);
    }
}
