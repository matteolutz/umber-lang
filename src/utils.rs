use crate::position::Position;

pub fn is_digit(c: &char) -> bool {
    *c as u32 >= 48 && *c as u32 <= 57
}

pub fn is_alpha(c: &char) -> bool {
    (*c as u32 >= 65 && *c as u32 <= 90) || (*c as u32 >= 97 && *c as u32 <= 122)
}

pub fn escape_char(c: &char) -> Option<char> {
    match c {
        '\'' => Some('\''),
        '\\' => Some('\\'),
        '0' => Some('\0'),
        'n' => Some('\n'),
        't' => Some('\t'),
        _ => None,
    }
}