use crate::data_types::chars::{Char16, Status};

#[repr(C)]
pub struct SimpleTextOutput {
    reset: unsafe extern "efiapi" fn(this: &SimpleTextOutput, extended_verification: bool) -> Status,
    output_string: unsafe extern "efiapi" fn(this: &SimpleTextOutput, output: *const Char16) -> Status,
    test_string: unsafe extern "efiapi" fn(this: &SimpleTextOutput, output: *const Char16) -> Status,
    query_mode: unsafe extern "efiapi" fn(this: &SimpleTextOutput, mode_number: usize, columns: *mut usize, rows: *mut usize) -> Status,
    set_mode: unsafe extern "efiapi" fn(this: &SimpleTextOutput, mode_number: usize) -> Status,
    set_attribute: unsafe extern "efiapi" fn(this: &SimpleTextOutput, attribute: usize) -> Status,
    clear_screen: unsafe extern "efiapi" fn(*mut SimpleTextOutput) -> Status,
    //TODO - set proper func signature
    set_cursor_position: unsafe extern "efiapi" fn() -> Status,
    //TODO - set proper func signature
    enable_cursor: unsafe extern "efiapi" fn() -> Status,
    //TODO - set proper func signature
    mode: unsafe extern "efiapi" fn() -> Status,
}

impl SimpleTextOutput {
    pub fn output_string(&mut self, string: &Char16) -> Status {
        unsafe {
            (self.output_string)(self, string);
            Status::SUCCESS
        }
    }

    pub fn clear_screen(&mut self) -> Status{
        unsafe{(self.clear_screen)(self)}.into()
    }
}