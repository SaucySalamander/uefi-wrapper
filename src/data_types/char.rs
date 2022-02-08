use core::ffi::c_void;
use core::ptr::NonNull;

#[derive(Clone, Copy, Default, Eq, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Char16(u16);

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Handle(NonNull<c_void>);

#[repr(C)]
pub enum Status {
    SUCCESS = 0,
    FAILURE = 1,
    //TODO add other error codes
}