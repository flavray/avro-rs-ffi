use std::ffi::CStr;
use std::fmt::Write;
use std::mem;
use std::os::raw::{c_char, c_uchar};
use std::ptr;
use std::slice;
use std::str;

use utils::{set_panic_hook, LAST_ERROR};

/// C-style error codes
#[repr(u32)]
pub enum AvroErrorCode {
    // no error
    NoError = 0,
    // panics and internals
    Panic = 1,
    Unknown = 3,
}

/// Initializes the library
#[no_mangle]
pub unsafe extern "C" fn avro_init() {
    set_panic_hook();
}

/// Returns the last error code.
///
/// If there is no error, 0 is returned.
#[no_mangle]
pub unsafe extern "C" fn avro_err_get_last_code() -> AvroErrorCode {
    LAST_ERROR.with(|e| {
        if (*e).borrow().is_some() {
            AvroErrorCode::Unknown
        } else {
            AvroErrorCode::NoError
        }
    })
}

/// Returns the last error message.
///
/// If there is no error an empty string is returned.  This allocates new memory
/// that needs to be freed with `avro_str_free`.
#[no_mangle]
pub unsafe extern "C" fn avro_err_get_last_message() -> AvroStr {
    LAST_ERROR.with(|e| {
        if let Some(ref err) = *e.borrow() {
            let mut msg = err.to_string();
            write!(&mut msg, "\n  caused by: {}", err.cause()).ok();
            AvroStr::from_string(msg)
        } else {
            Default::default()
        }
    })
}

/// Returns the panic information as string.
#[no_mangle]
pub unsafe extern "C" fn avro_err_get_backtrace() -> AvroStr {
    LAST_ERROR.with(|e| {
        if let Some(ref error) = *e.borrow() {
            let mut out = format!("stacktrace: {}", error.backtrace()).to_owned();
            AvroStr::from_string(out)
        } else {
            Default::default()
        }
    })
}

/// Clears the last error.
#[no_mangle]
pub unsafe extern "C" fn avro_err_clear() {
    LAST_ERROR.with(|e| {
        *e.borrow_mut() = None;
    });
}

/// Represents a string.
#[repr(C)]
#[derive(Debug)]
pub struct AvroStr {
    pub data: *mut c_char,
    pub len: usize,
    pub owned: bool,
}

impl Default for AvroStr {
    fn default() -> AvroStr {
        AvroStr {
            data: ptr::null_mut(),
            len: 0,
            owned: false,
        }
    }
}

impl AvroStr {
    pub fn new(s: &str) -> AvroStr {
        AvroStr {
            data: s.as_ptr() as *mut c_char,
            len: s.len(),
            owned: false,
        }
    }

    pub fn from_string(mut s: String) -> AvroStr {
        s.shrink_to_fit();
        let rv = AvroStr {
            data: s.as_ptr() as *mut c_char,
            len: s.len(),
            owned: true,
        };
        mem::forget(s);
        rv
    }

    pub unsafe fn free(&mut self) {
        if self.owned {
            String::from_raw_parts(self.data as *mut _, self.len, self.len);
            self.data = ptr::null_mut();
            self.len = 0;
            self.owned = false;
        }
    }

    pub fn as_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(slice::from_raw_parts(self.data as *const _, self.len)) }
    }

    pub fn into_string(self) -> String {
        unsafe {
            String::from_utf8_unchecked(Vec::from_raw_parts(
                self.data as *mut _,
                self.len,
                self.len,
            ))
        }
    }
}

ffi_fn! {
    /// Creates a avro str from a c string.
    ///
    /// This sets the string to owned. In case it's not owned you either have
    /// to make sure you are not freeing the memory or you need to set the
    /// owned flag to false.
    unsafe fn avro_str_from_c_str(s: *const c_char) -> Result<AvroStr> {
        let s = CStr::from_ptr(s).to_str()?;
        Ok(AvroStr {
            data: s.as_ptr() as *mut _,
            len: s.len(),
            owned: true,
        })
    }
}

/// Frees a avro str.
///
/// If the string is marked as not owned then this function does not
/// do anything.
#[no_mangle]
pub unsafe extern "C" fn avro_str_free(s: *mut AvroStr) {
    if !s.is_null() {
        (*s).free()
    }
}

/// Represents a byte array.
#[repr(C)]
#[derive(Debug)]
pub struct AvroByteArray {
    pub data: *mut c_uchar,
    pub len: usize,
    pub owned: bool,
}

impl Default for AvroByteArray {
    fn default() -> AvroByteArray {
        AvroByteArray {
            data: ptr::null_mut(),
            len: 0,
            owned: false,
        }
    }
}

impl AvroByteArray {
    pub fn new(s: &[u8]) -> AvroByteArray {
        AvroByteArray {
            data: s.as_ptr() as *mut c_uchar,
            len: s.len(),
            owned: false,
        }
    }

    pub fn from_vec_u8(mut v: Vec<u8>) -> AvroByteArray {
        v.shrink_to_fit();
        let rv = AvroByteArray {
            data: v.as_ptr() as *mut c_uchar,
            len: v.len(),
            owned: true,
        };
        mem::forget(v);
        rv
    }

    pub unsafe fn free(&mut self) {
        if self.owned {
            Vec::from_raw_parts(self.data as *mut _, self.len, self.len);
            self.data = ptr::null_mut();
            self.len = 0;
            self.owned = false;
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.data as *const _, self.len) }
    }

    pub unsafe fn into_vec_u8(self) -> Vec<u8> {
        Vec::from_raw_parts(self.data as *mut _, self.len, self.len)
    }
}

ffi_fn! {
    /// Creates a avro byte array from a c string.
    ///
    /// This sets the array to owned.  In case it's not owned you either have
    /// to make sure you are not freeing the memory or you need to set the
    /// owned flag to false.
    unsafe fn avro_byte_array_from_c_array(a: *const c_uchar, len: usize) -> Result<AvroByteArray> {
        Ok(AvroByteArray {
            data: a as *mut _,
            len: len as usize,
            owned: true,
        })
    }
}

/// Frees a avro byte array.
///
/// If the array is marked as not owned then this function does not
/// do anything.
#[no_mangle]
pub unsafe extern "C" fn avro_byte_array_free(a: *mut AvroByteArray) {
    if !a.is_null() {
        (*a).free()
    }
}
