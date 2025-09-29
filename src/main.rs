use crate::lexicon::dictionary::Dictionary;
use crate::lexicon::vocab_pair::VocabPair;
use crate::lexicon::words::WordsPool;
use crate::setting::settings::Settings;
use env_logger::Builder;
use inquire::{Confirm, Select};
use log::LevelFilter;
use rand::Rng;
use rand::seq::{IndexedRandom, SliceRandom};
use std::error::Error;
use std::path::Path;
use std::str::FromStr;

mod lexicon;
mod setting;

fn main() -> Result<(), Box<dyn Error>> {
    let amount_elements = 3;
    let mut rng = rand::rng();

    let settings = Settings::new("config.yaml", "APP")
        .map_err(|err| format!("Failed to load setting: {err}"))?;

    Builder::new()
        .filter_level(
            LevelFilter::from_str(settings.logging.log_level.as_str()).unwrap_or(LevelFilter::Info),
        )
        .init();

    let words_path = Path::new(&settings.vocab.words_path);
    let dictionary_path = Path::new(&settings.vocab.dictionary_path);

    let words_pool = WordsPool::new(words_path);
    let dictionary = Dictionary::new(dictionary_path);

    let words = words_pool.get_words().unwrap();
    let mut pairs = dictionary.get_pairs();
    pairs.shuffle(&mut rng);

    if pairs.is_empty() || words.is_empty() {
        log::error!("File is empty");
        return Ok(());
    }

    loop {
        let correct_answer = run_quiz(&mut pairs, &words, &mut rng, amount_elements);

        println!("Результат: {}/{} слов.\n", correct_answer, pairs.len());

        if ask_restart() {
            pairs.shuffle(&mut rng);
            println!();
            continue;
        } else {
            break;
        }
    }

    Ok(())
}

fn run_quiz(
    pairs: &mut Vec<VocabPair>,
    words: &Vec<String>,
    rng: &mut impl Rng,
    amount_elements: usize,
) -> usize {
    let mut correct: usize = 0;

    for (idx, pair) in pairs.iter().enumerate() {
        let word = pair.get_word();
        let translate = pair.get_translate();

        let choices = make_choices(&translate, &words, rng, amount_elements);
        let answer = Select::new(
            &format!("{}. Перевод слова \"{}\" ->", idx + 1, word),
            choices.clone(),
        )
        .prompt()
        .unwrap_or_else(|_| std::process::exit(0));

        if answer == translate {
            println!("✅ Верно!\n");
            correct += 1;
        } else {
            println!("❌ Неверно!\n");
        }
    }

    correct
}

fn ask_restart() -> bool {
    match Confirm::new("Пройти тест заново?")
        .with_default(false)
        .prompt()
    {
        Ok(true) => true,
        _ => false,
    }
}

fn make_choices<'a>(
    right: &'a str,
    pool: &'a [String],
    rng: &mut impl rand::Rng,
    amount_elements: usize,
) -> Vec<&'a str> {
    let mut choices: Vec<&str> = pool
        .choose_multiple(rng, amount_elements)
        .map(String::as_str)
        .collect();

    choices.push(right);
    choices.shuffle(rng);
    choices
}
