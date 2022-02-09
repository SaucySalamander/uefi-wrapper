use core::ffi::c_void;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Char8(u8);

impl TryFrom<char> for Char8 {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let u16char = value as u16;
        if u16char <= 0xff {
            Ok(Char8(u16char as u8))
        } else {
            Err(())
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Char16(u16);

impl TryFrom<char> for Char16 {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let u32char = value as u32;
        if u32char <= 0xffff {
            Ok(Char16(u32char as u16))
        } else {
            Err(())
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Handle(*mut c_void);

#[repr(C)]
pub enum Status {
    SUCCESS = 0,
    FAILURE = 1,
    //TODO add other error codes
}