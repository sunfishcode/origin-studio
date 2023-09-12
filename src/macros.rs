/// Prints to the standard output.
#[macro_export]
macro_rules! print {
    ($($args:tt)*) => {
        core::fmt::write(&mut $crate::io::stdout(), format_args!($($args)*)).unwrap();
    };
}

/// Prints to the standard output, with a newline.
#[macro_export]
macro_rules! println {
    () => {
        <$crate::io::Stdout as core::fmt::Write>::write_str(&mut $crate::io::stdout(), "\n").unwrap();
    };
    ($format:expr) => {
        <$crate::io::Stdout as core::fmt::Write>::write_str(&mut $crate::io::stdout(), concat!($format, "\n")).unwrap();
    };
    ($format:expr, $($args:tt)*) => {
        core::fmt::write(&mut $crate::io::stdout().lock(), format_args!(concat!($format, "\n"), $($args)*)).unwrap();
    };
}

/// Prints to the standard error.
#[macro_export]
macro_rules! eprint {
    ($($args:tt)*) => {
        core::fmt::write(&mut $crate::io::stderr(), format_args!($($args)*)).unwrap();
    };
}

/// Prints to the standard error< with a newline.
#[macro_export]
macro_rules! eprintln {
    () => {
        <$crate::io::Stderr as core::fmt::Write>::write_str(&mut $crate::io::stderr(), "\n").unwrap();
    };
    ($format:expr) => {
        <$crate::io::Stderr as core::fmt::Write>::write_str(&mut $crate::io::stderr(), concat!($format, "\n")).unwrap();
    };
    ($format:expr, $($args:tt)*) => {
        core::fmt::write(&mut $crate::io::stderr().lock(), format_args!(concat!($format, "\n"), $($args)*)).unwrap();
    };
}

/// Writes formatted data into a buffer.
#[macro_export]
macro_rules! write {
    ($dst:expr, $($arg:tt)*) => {
        $dst.write_fmt($crate::format_args!($($arg)*))
    };
}

/// Writes formatted data into a buffer, with a newline appended.
#[macro_export]
macro_rules! writeln {
    ($dst:expr $(,)?) => {
        $crate::write!($dst, "\n")
    };
    ($dst:expr, $($arg:tt)*) => {
        $dst.write_fmt(format_args!(concat!($format, "\n"), $($args)*))
    };
}
