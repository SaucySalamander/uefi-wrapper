use core::ffi::c_void;

use crate::data_types::{guid::Guid, Handle, Status};

#[repr(C)]
pub struct BootServices {
    open_protocol: extern "efiapi" fn(
        handle: Handle,
        protocol: *mut Guid,
        interface: *mut *mut c_void,
        agent_handle: Handle,
        controller_handle: Handle,
        attributes: u32,
    ) -> Status,
    close_protocol: extern "efiapi" fn(
        handle: Handle,
        protocol: *mut Guid,
        agent_handle: Handle,
        controller_handle: Handle,
    ) -> Status,
}
