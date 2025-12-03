use anyhow::Error;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct WordsPool {
    words: Vec<String>,
}

impl WordsPool {
    pub fn new(path: &Path) -> Result<Self, Error> {
        let words = Self::load_words(path)?;

        Ok(WordsPool { words })
    }

    pub fn get_words(&self) -> &[String] {
        &self.words
    }

    fn load_words(path: &Path) -> Result<Vec<String>, Error> {
        let data = fs::read_to_string(path)?;

        Ok(data
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(String::from)
            .collect())
    }
}
