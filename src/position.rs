#[derive(Clone)]
pub struct Position {
    index: usize,
    line: usize,
    col: usize,
    file_name: Box<String>,
    file_text: Box<String>,
}

impl Position {

    pub fn new(file_name: Box<String>, file_text: Box<String>) -> Self {
        Position {
            line: 0,
            col: 0,
            index: 0,
            file_name,
            file_text
        }
    }

    pub fn empty() -> Self {
        Position {
            line: 0,
            col: 0,
            index: 0,
            file_name: Box::new("".to_string()),
            file_text: Box::new("".to_string())
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

    pub fn file_name(&self) -> &Box<String> {
        &self.file_name
    }

    pub fn file_text(&self) -> &Box<String> {
        &self.file_text
    }

}