#[macro_export]
macro_rules! print {
    ($($args:tt)*) => {
        core::fmt::write(&mut $crate::std::io::stdout(), format_args!($($args)*)).unwrap();
    };
}

#[macro_export]
macro_rules! println {
    () => {
        <$crate::std::io::Stdout as core::fmt::Write>::write_str(&mut $crate::std::io::stdout(), "\n").unwrap();
    };
    ($format:expr) => {
        <$crate::std::io::Stdout as core::fmt::Write>::write_str(&mut $crate::std::io::stdout(), concat!($format, "\n")).unwrap();
    };
    ($format:expr, $($args:tt)*) => {
        core::fmt::write(&mut $crate::std::io::stdout().lock(), format_args!(concat!($format, "\n"), $($args)*)).unwrap();
    };
}

#[macro_export]
macro_rules! eprint {
    ($($args:tt)*) => {
        core::fmt::write(&mut $crate::std::io::stderr(), format_args!($($args)*)).unwrap();
    };
}

#[macro_export]
macro_rules! eprintln {
    () => {
        <$crate::std::io::Stderr as core::fmt::Write>::write_str(&mut $crate::std::io::stderr(), "\n").unwrap();
    };
    ($format:expr) => {
        <$crate::std::io::Stderr as core::fmt::Write>::write_str(&mut $crate::std::io::stderr(), concat!($format, "\n")).unwrap();
    };
    ($format:expr, $($args:tt)*) => {
        core::fmt::write(&mut $crate::std::io::stderr().lock(), format_args!(concat!($format, "\n"), $($args)*)).unwrap();
    };
}

#[macro_export]
macro_rules! write {
    ($dst:expr, $($arg:tt)*) => {
        $dst.write_fmt($crate::format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! writeln {
    ($dst:expr $(,)?) => {
        $crate::write!($dst, "\n")
    };
    ($dst:expr, $($arg:tt)*) => {
        $dst.write_fmt(format_args!(concat!($format, "\n"), $($args)*))
    };
}
