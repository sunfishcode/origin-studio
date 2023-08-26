use core::ffi::CStr;
use core::ptr::null_mut;
use core::slice;
use core::str;

pub(crate) struct MainArgs {
    pub(crate) argc: i32,
    pub(crate) argv: *mut *mut u8,
    pub(crate) envp: *mut *mut u8,
}

pub(crate) static mut MAIN_ARGS: MainArgs = MainArgs {
    argc: 0,
    argv: null_mut(),
    envp: null_mut(),
};

unsafe impl Send for MainArgs {}
unsafe impl Sync for MainArgs {}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum VarError {
    NotPresent,
    NotUnicode,
}

pub fn var<K: AsRef<str>>(key: K) -> Result<&'static str, VarError> {
    unsafe {
        let mut ptr = MAIN_ARGS.envp;
        let key_bytes = key.as_ref().as_bytes();
        loop {
            let env = *ptr;
            if env.is_null() {
                break;
            }
            let mut c = env;
            while *c != b'=' {
                c = c.add(1);
            }
            if key_bytes
                == slice::from_raw_parts(env.cast::<u8>(), c.offset_from(env).try_into().unwrap())
            {
                return str::from_utf8(CStr::from_ptr(c.add(1).cast()).to_bytes())
                    .map_err(|_| VarError::NotUnicode);
            }
            ptr = ptr.add(1);
        }
    }
    Err(VarError::NotPresent)
}

pub fn args() -> Args {
    Args { pos: 0 }
}

pub struct Args {
    pos: usize,
}

impl Iterator for Args {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.pos < MAIN_ARGS.argc as usize {
                let arg = MAIN_ARGS.argv.add(self.pos);
                self.pos += 1;
                let cstr = CStr::from_ptr(arg.read().cast());
                return Some(core::str::from_utf8(cstr.to_bytes()).unwrap());
            }
        }
        None
    }
}

impl ExactSizeIterator for Args {
    fn len(&self) -> usize {
        unsafe { MAIN_ARGS.argc as usize - self.pos }
    }
}
