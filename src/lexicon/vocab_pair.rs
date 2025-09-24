#[derive(Debug, Clone)]
pub struct VocabPair {
    word: String,
    translate: String,
}

impl VocabPair {
    pub fn new(word: String, translate: String) -> Self {
        VocabPair { word, translate }
    }

    pub fn get_word(&self) -> String {
        self.word.clone()
    }

    pub fn get_translate(&self) -> String {
        self.translate.clone()
    }
}
