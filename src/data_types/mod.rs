use core::ffi::c_void;

pub mod chars;
pub mod guid;
pub mod strings;

#[repr(C)]
pub enum Status {
    SUCCESS = 0,
    FAILURE = 1,
    //TODO add other error codes
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Handle(*mut c_void);
