use core::fmt;

#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone)]
pub struct Location {
    file: String,
    line: usize,
    col: usize,
}

impl Location {
    pub fn incr_col(&mut self) {
        self.col += 1;
    }

    pub fn incr_line(&mut self) {
        self.line += 1;
    }

    pub fn into_past(self, offset: usize) -> Self {
        Self {
            col: self.col.checked_sub(offset).unwrap_or(1),
            ..self
        }
    }

    pub fn new_line(&mut self) {
        self.line += 1;
        self.col = 1;
    }

    pub fn get(self) -> (String, usize, usize) {
        (self.file, self.line, self.col)
    }
}

impl From<&str> for Location {
    fn from(value: &str) -> Self {
        Self {
            file: value.to_owned(),
            line: 1,
            col: 1,
        }
    }
}

impl From<String> for Location {
    fn from(value: String) -> Self {
        Self {
            file: value,
            line: 1,
            col: 1,
        }
    }
}

#[expect(clippy::min_ident_chars)]
impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.col)
    }
}
