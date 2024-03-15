use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq)]
pub enum ValueSize {
    Byte,
    Word,
    Dword,
    Qword,
}

impl ValueSize {
    pub fn get_size_in_bytes(&self) -> u8 {
        match self {
            ValueSize::Byte => 1,
            ValueSize::Word => 2,
            ValueSize::Dword => 4,
            ValueSize::Qword => 8,
        }
    }
}

impl Display for ValueSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueSize::Byte => write!(f, "BYTE"),
            ValueSize::Word => write!(f, "WORD"),
            ValueSize::Dword => write!(f, "DWORD"),
            ValueSize::Qword => write!(f, "QWORD"),
        }
    }
}
