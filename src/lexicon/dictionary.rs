use crate::lexicon::vocab_pair::VocabPair;
use anyhow::Error;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Dictionary {
    pairs: Vec<VocabPair>,
}

impl Dictionary {
    pub fn new(path: &Path) -> Self {
        let pairs = Self::load_pairs(path).unwrap();

        Dictionary { pairs }
    }

    pub fn get_pairs(self) -> Vec<VocabPair> {
        self.pairs
    }

    fn load_pairs(path: &Path) -> Result<Vec<VocabPair>, Error> {
        let data = fs::read_to_string(path)?;

        Ok(data
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .filter_map(|line| line.split_once(':'))
            .map(|(word, translate)| {
                VocabPair::new(word.trim().to_string(), translate.trim().to_string())
            })
            .collect::<Vec<_>>())
    }
}
