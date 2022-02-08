use crate::data_types::char::{Char16, Handle};
use crate::protocols::simple_text_input::SimpleTextInput;
use crate::protocols::simple_text_output::SimpleTextOutput;

#[repr(C)]
pub struct TableHeader {
    signature: u64,
    revision: u32,
    header_size: u32,
    crc32: u32,
    reserved: u32,
}

#[repr(C)]
pub struct SystemTable {
    pub(crate) header: TableHeader,
    pub(crate) firmware_vendor: *mut Char16,
    pub(crate) firmware_revision: u32,
    pub(crate) console_in_handle: Handle,
    pub(crate) console_in: *mut SimpleTextInput,
    pub(crate) console_out_handle: Handle,
    pub(crate) console_out: *mut SimpleTextOutput,
    //TODO - Fill out rest of struct

    // EFI_HANDLE StandardErrorHandle,
    // EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL *StdErr,
    // EFI_RUNTIME_SERVICES *RuntimeServices,
    // EFI_BOOT_SERVICES *BootServices,
    // UINTN NumberOfTableEntries,
    // EFI_CONFIGURATION_TABLE *ConfigurationTable,
}

impl SystemTable {
    //TODO rename to keep in line with naming uefi naming convention
    pub fn stdout(&mut self) -> &mut SimpleTextOutput {
        unsafe { &mut *self.console_out.cast() }
    }
}