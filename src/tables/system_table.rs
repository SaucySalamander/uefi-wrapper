use crate::data_types::char::{Char16, Handle};
use crate::protocols::simple_text_input::SimpleTextInput;
use crate::protocols::simple_text_output::SimpleTextOutput;

#[repr(C)]
pub struct TableHeader {
    pub signature: u64,
    pub revision: u32,
    pub header_size: u32,
    pub crc32: u32,
    pub reserved: u32,
}

#[repr(C)]
pub struct SystemTable {
    pub header: TableHeader,
    pub firmware_vendor: *mut Char16,
    pub firmware_revision: u32,
    pub console_in_handle: Handle,
    pub console_in: *mut SimpleTextInput,
    pub console_out_handle: Handle,
    pub console_out: *mut SimpleTextOutput,
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