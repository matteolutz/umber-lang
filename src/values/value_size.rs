use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq)]
pub enum ValueSize {
    BYTE,
    WORD,
    DWORD,
    QWORD,
}

impl ValueSize {
    pub fn get_size_in_bytes(&self) -> u8 {
        match self {
            ValueSize::BYTE => 1,
            ValueSize::WORD => 2,
            ValueSize::DWORD => 4,
            ValueSize::QWORD => 8,
        }
    }
}

impl Display for ValueSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueSize::BYTE => write!(f, "BYTE"),
            ValueSize::WORD => write!(f, "WORD"),
            ValueSize::DWORD => write!(f, "DWORD"),
            ValueSize::QWORD => write!(f, "QWORD"),
        }
    }
}