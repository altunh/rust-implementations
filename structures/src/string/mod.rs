use std::str::Utf8Error;

use super::vector::Vec;

#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct String {
    vec: Vec<u8>,
}

pub struct FromUtf8Error {
    bytes: Vec<u8>,
    error: Utf8Error
}

impl String {
    pub fn new() -> Self {
        String { vec: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        String { vec: Vec::with_capacity(capacity) }
    }

    pub fn from_utf8(vec: Vec<u8>) -> Result<String, FromUtf8Error> {
        match std::str::from_utf8(&vec) {
            Ok(..) => Ok(String { vec }),
            Err(e) => Err(FromUtf8Error { bytes: vec, error: e })
        }
    }

    pub unsafe fn from_raw_parts(buf: *mut u8, length: usize, capacity: usize) -> String {
        unsafe {
            String { vec: Vec::from_raw_parts(buf,length, capacity) }
        }
    }
}

