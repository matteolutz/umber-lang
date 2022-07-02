use std::path::{Path, PathBuf};
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct Position {
    index: usize,
    line: usize,
    col: usize,
    file_path: PathBuf
}

impl Position {

    pub fn new(file_path: PathBuf) -> Self {
        Position {
            line: 0,
            col: 0,
            index: 0,
            file_path,
        }
    }

    pub fn empty() -> Self {
        Position {
            line: 0,
            col: 0,
            index: 0,
            file_path: PathBuf::new(),
        }
    }

    pub fn advance(&mut self, current_char: &char) {
        self.index += 1;
        self.col += 1;

        if current_char == &'\n' {
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

    pub fn file_name(&self) -> &PathBuf {
        &self.file_path
    }

}