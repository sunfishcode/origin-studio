#[cfg(feature = "thread")]
use crate::std::thread::{ReentrantMutex, ReentrantMutexGuard};
use core::fmt::{self, Arguments};
#[cfg(not(feature = "thread"))]
use core::marker::PhantomData;

pub type Error = rustix::io::Errno;

pub type Result<T> = core::result::Result<T, Error>;

pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;

    fn is_read_vectored(&self) -> bool {
        false
    }
}

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;

    fn is_write_vectored(&self) -> bool {
        false
    }

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

pub struct StdoutLock<'a> {
    #[cfg(feature = "thread")]
    _lock: ReentrantMutexGuard<'a, ()>,

    #[cfg(not(feature = "thread"))]
    _phantom: PhantomData<&'a ()>,
}

#[cfg(feature = "thread")]
static STDOUT_LOCK: ReentrantMutex<()> = ReentrantMutex::new(());

pub struct Stdout(());

pub fn stdout() -> Stdout {
    Stdout(())
}

impl Stdout {
    pub fn lock(&self) -> StdoutLock<'static> {
        StdoutLock {
            #[cfg(feature = "thread")]
            _lock: STDOUT_LOCK.lock(),

            #[cfg(not(feature = "thread"))]
            _phantom: PhantomData,
        }
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
        Stdout(()).write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        Stdout(()).flush()
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
        Stdout(()).write_str(s)
    }
}

pub struct StderrLock<'a> {
    #[cfg(feature = "thread")]
    _lock: ReentrantMutexGuard<'a, ()>,

    #[cfg(not(feature = "thread"))]
    _phantom: PhantomData<&'a ()>,
}

#[cfg(feature = "thread")]
static STDERR_LOCK: ReentrantMutex<()> = ReentrantMutex::new(());

pub struct Stderr(());

pub fn stderr() -> Stderr {
    Stderr(())
}

impl Stderr {
    pub fn lock(&self) -> StderrLock<'static> {
        StderrLock {
            #[cfg(feature = "thread")]
            _lock: STDERR_LOCK.lock(),

            #[cfg(not(feature = "thread"))]
            _phantom: PhantomData,
        }
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
        Stderr(()).write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        Stderr(()).flush()
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
        Stderr(()).write_str(s)
    }
}

pub trait Seek {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64>;

    fn rewind(&mut self) -> Result<()> {
        self.seek(SeekFrom::Start(0))?;
        Ok(())
    }

    fn stream_position(&mut self) -> Result<u64> {
        self.seek(SeekFrom::Current(0))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum SeekFrom {
    Start(u64),
    End(i64),
    Current(i64),
}
