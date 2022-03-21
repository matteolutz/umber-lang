use crate::position::Position;

pub fn string_with_arrows(text: &str, pos_start: &Position, pos_end: &Position) -> String {
    let mut result = String::new();

    let mut idx_start = text[0..*pos_start.index()].rfind('\n').unwrap_or(0);
    let mut idx_end = text[(idx_start + 1)..].find('\n').unwrap_or(text.len()) + (idx_start + 1);

    let line_count = pos_end.line() - pos_start.line() + 1;

    for i in 0..line_count {
        println!("idx_start: {} | idx_end: {}", &idx_start, &idx_end);
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