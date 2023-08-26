#![no_std]
#![no_main]
origin_studio::no_problem!();

use std::io::Write;

fn main() {
    for i in std::env::args() {
        writeln!(std::io::stdout(), "arg {}", i).unwrap();
    }
    writeln!(std::io::stdout(), "HOME {}", std::env::var("HOME").unwrap()).unwrap();
}
