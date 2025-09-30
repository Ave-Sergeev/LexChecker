use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Debug)]
pub enum TestMode {
    Express(usize),
    Full,
}

impl Display for TestMode {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        match self {
            Self::Express(size) => write!(fmt, "Экспресс: {size} слов"),
            Self::Full => write!(fmt, "Полный тест"),
        }
    }
}

impl TestMode {
    pub fn size(self, pairs_len: usize) -> usize {
        match self {
            TestMode::Express(n) => n.min(pairs_len),
            TestMode::Full => pairs_len,
        }
    }
}
