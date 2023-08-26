use crate::std::thread::{ReentrantMutex, ReentrantMutexGuard};
use core::fmt::{self, Arguments};

pub type Error = rustix::io::Errno;

pub type Result<T> = core::result::Result<T, Error>;

pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
}

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;

    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, mut buf: &[u8]) -> Result<()> {
        while !buf.is_empty() {
            match self.write(buf) {
                Ok(n) => buf = &buf[n..],
                Err(Error::INTR) => {}
                Err(err) => return Err(err),
            }
        }
        Ok(())
    }

    fn write_fmt(&mut self, fmt: Arguments<'_>) -> Result<()> {
        // Create a shim which translates a Write to a fmt::Write and saves
        // off I/O errors. instead of discarding them
        struct Adapter<'a, T: ?Sized + 'a> {
            inner: &'a mut T,
            error: Result<()>,
        }

        impl<T: Write + ?Sized> fmt::Write for Adapter<'_, T> {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                match self.inner.write_all(s.as_bytes()) {
                    Ok(()) => Ok(()),
                    Err(e) => {
                        self.error = Err(e);
                        Err(fmt::Error)
                    }
                }
            }
        }

        let mut output = Adapter {
            inner: self,
            error: Ok(()),
        };
        match fmt::write(&mut output, fmt) {
            Ok(()) => Ok(()),
            Err(..) => {
                // check if the error came from the underlying `Write` or not
                if output.error.is_err() {
                    output.error
                } else {
                    Err(Error::IO)
                }
            }
        }
    }
}

pub struct StdoutLock<'a>(ReentrantMutexGuard<'a, ()>);

static STDOUT_LOCK: ReentrantMutex<()> = ReentrantMutex::new(());

pub struct Stdout(());

pub fn stdout() -> Stdout {
    Stdout(())
}

impl Stdout {
    pub fn lock(&self) -> StdoutLock<'static> {
        StdoutLock(STDOUT_LOCK.lock())
    }
}

impl Write for Stdout {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        rustix::io::write(unsafe { rustix::stdio::stdout() }, buf)
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl<'a> Write for StdoutLock<'a> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        Stdout(*self.0).write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        Stdout(*self.0).flush()
    }
}

impl core::fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        match self.write_all(s.as_bytes()) {
            Ok(_) => Ok(()),
            Err(err) => panic!("failed printing to stdout: {:?}", err),
        }
    }
}

impl<'a> core::fmt::Write for StdoutLock<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        Stdout(*self.0).write_str(s)
    }
}

pub struct StderrLock<'a>(ReentrantMutexGuard<'a, ()>);

static STDERR_LOCK: ReentrantMutex<()> = ReentrantMutex::new(());

pub struct Stderr(());

pub fn stderr() -> Stderr {
    Stderr(())
}

impl Stderr {
    pub fn lock(&self) -> StderrLock<'static> {
        StderrLock(STDERR_LOCK.lock())
    }
}

impl Write for Stderr {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        rustix::io::write(unsafe { rustix::stdio::stderr() }, buf)
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl<'a> Write for StderrLock<'a> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        Stderr(*self.0).write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        Stderr(*self.0).flush()
    }
}

impl core::fmt::Write for Stderr {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        match self.write_all(s.as_bytes()) {
            Ok(_) => Ok(()),
            Err(err) => panic!("failed printing to stderr: {:?}", err),
        }
    }
}

impl<'a> core::fmt::Write for StderrLock<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        Stderr(*self.0).write_str(s)
    }
}
