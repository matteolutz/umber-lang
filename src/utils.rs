use crate::position::Position;

pub fn string_with_arrows(text: &str, pos_start: &Position, pos_end: &Position) -> String {
    let mut result = String::new();

    let mut idx_start = text[0..*pos_start.index()].rfind('\n').unwrap_or(0);
    let mut idx_end = text[(idx_start + 1)..].find('\n').unwrap_or(text.len()) + (idx_start + 1);

    let line_count = pos_end.line() - pos_start.line() + 1;

    for i in 0..line_count {
        let line = &text[idx_start..idx_end];

        let col_start = if i == 0 { *pos_start.col() } else { 0 };
        let col_end = if i == line_count - 1 { *pos_end.col() } else { line.len() - 1 };

        result.push_str(line);
        result.push('\n');

        for _s in 0..col_start {
            result.push(' ');
        }
        for _s in col_start..col_end {
            result.push('^');
        }

        idx_start = idx_end;
        idx_end = text[idx_start + 1..].find('\n').unwrap_or(text.len());
    }

    return result;
}

pub fn is_digit(c: &char) -> bool {
    *c as u32 >= 48 && *c as u32 <= 57
}

pub fn is_alpha(c: &char) -> bool {
    (*c as u32 >= 65 && *c as u32 <= 90) || (*c as u32 >= 97 && *c as u32 <= 122)
}

// TODO: check how or if you need escaped characters
pub fn should_escape_char(c: &char) -> Option<char> {
    return None;

    match c {
        'n' => Some('\n'),
        't' => Some('\t'),
        _ => None,
    }
}