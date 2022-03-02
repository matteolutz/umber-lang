#[derive(Copy, Clone)]
pub struct Position {
    index: usize,
    line: usize,
    col: usize,
    file_name: &'static str,
    file_text: &'static str,
}

impl Position {

    pub fn new(file_name: &'static str, file_text: &'static str) -> Self {
        Position {
            line: 0,
            col: 0,
            index: 0,
            file_name,
            file_text
        }
    }

    pub fn advance(&mut self, current_char: char) {
        self.index += 1;
        self.col += 1;

        if current_char == '\n' {
            self.line += 1;
            self.col = 0;
        }
    }

    pub fn index(&self) -> &usize {
        &self.index
    }

    pub fn line(&self) -> &usize {
        &self.line
    }

    pub fn col(&self) -> &usize {
        &self.col
    }

    pub fn file_name(&self) -> &&'static str {
        &self.file_name
    }

    pub fn file_text(&self) -> &&'static str {
        &self.file_text
    }

}