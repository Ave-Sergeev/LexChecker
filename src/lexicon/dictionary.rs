use crate::lexicon::vocab_pair::VocabPair;
use anyhow::Error;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Dictionary {
    pairs: Vec<VocabPair>,
}

impl Dictionary {
    pub fn new(path: &Path) -> Result<Self, Error> {
        let pairs = Self::load_pairs(path)?;

        Ok(Dictionary { pairs })
    }

    pub fn get_pairs_mut(&mut self) -> &mut [VocabPair] {
        &mut self.pairs
    }

    fn load_pairs(path: &Path) -> Result<Vec<VocabPair>, Error> {
        let data = fs::read_to_string(path)?;

        Ok(data
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                if line.is_empty() || line.starts_with("//") {
                    return None;
                }
                line.split_once(':').map(|(word, translate)| {
                    VocabPair::new(word.trim().to_owned(), translate.trim().to_owned())
                })
            })
            .collect::<Vec<_>>())
    }
}
