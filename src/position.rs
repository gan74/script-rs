
use std::cmp;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Position {
    pub filename: String,
    pub line: usize,
    col: usize
}

impl Position {
    pub fn new(filename: &str) -> Position {
        Position {
            filename: String::from(filename),
            line: 0,
            col: 0
        }
    }

    pub fn eof() -> Position {
         Position {
            filename: String::new(),
            line: usize::max_value(),
            col: usize::max_value()
        }
    }

    pub fn next_line(&mut self) {
        self.line += 1;
        self.col = 0;
    }

    pub fn next_col(&mut self) {
        self.col += 1;
    }

    pub fn is_eof(&self) -> bool {
        return self.line == usize::max_value();
    }

    // NOT EFFICIENT, DONT SPAM
    pub fn pos_string<'a>(&self, text: &'a str) -> String {
        let mut lines = self.line;
        let mut chars = text.chars();
        while lines != 0 {
            if let Some(c) = chars.next() {
                if c == '\n' {
                    lines = lines - 1
                }
            } else {
                break;
            }
        }
        let line = chars.as_str();
        let len = cmp::min(line.len(), line.find('\n').unwrap_or(line.len()));
        let col = cmp::min(len, self.col);
        let mut cursor = String::with_capacity(col + 1);
        for _ in 0..col {
            cursor.push('~');
        }
        cursor.push('^');
        String::from(&line[..len]) + "\n" + cursor.as_str()
    }
}


impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_eof() {
            write!(f, "at EOF")
        } else {
            write!(f, "in {} at line {}", self.filename, self.line + 1)
        }
    }
}