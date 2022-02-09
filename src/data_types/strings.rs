use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use crate::data_types::chars::Char16;

pub struct CString16(Vec<Char16>);

impl TryFrom<String> for CString16 {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut container: Vec<Char16> = vec!();
        for char in value.chars() {
            let convert = Char16::try_from(char);

            if convert.is_ok() {
                container.push(convert.unwrap());
            }
        }

        let char16null = Char16::try_from(0 as char);
        container.push(char16null.unwrap());
        Ok(CString16(container))
    }
}