use crate::lexicon::dictionary::Dictionary;
use crate::lexicon::vocab_pair::VocabPair;
use crate::lexicon::words::WordsPool;
use crate::mode::test_mode::TestMode;
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
mod mode;
mod setting;

fn main() -> Result<(), Box<dyn Error>> {
    let min_words_required: usize = 3;

    let mut rng = rand::rng();

    let settings = Settings::new("config.yaml", "APP")
        .map_err(|err| format!("Failed to load setting: {err}"))?;

    Builder::new()
        .filter_level(
            LevelFilter::from_str(settings.logging.log_level.as_str()).unwrap_or(LevelFilter::Info),
        )
        .init();

    let amount_incorrect_answer = settings.test.amount_incorrect_answer;

    let words_path = Path::new(&settings.vocab.words_path);
    let dictionary_path = Path::new(&settings.vocab.dictionary_path);

    let words_pool = WordsPool::new(words_path)?;
    let mut dictionary = Dictionary::new(dictionary_path)?;

    let words = words_pool.get_words();
    let pairs = dictionary.get_pairs_mut();

    if pairs.is_empty() {
        return Err("Dictionary file is empty".into());
    }

    if words.len() < min_words_required {
        return Err(format!(
            "Not enough words in the [random word pool] file. Required: {}, available: {}",
            min_words_required,
            words.len()
        )
        .into());
    }

    loop {
        pairs.shuffle(&mut rng);

        let total_questions = choose_test_size(pairs.len());
        println!();

        let quiz_pairs = pairs
            .iter()
            .take(total_questions)
            .cloned()
            .collect::<Vec<_>>();
        let correct_answers = run_quiz(&quiz_pairs, words, &mut rng, amount_incorrect_answer);

        println!(
            "\nРезультат: {correct_answers}/{total_questions} слов (≈{}%)\n",
            (correct_answers * 100) / total_questions
        );

        if !ask_restart() {
            break;
        }
        println!();
    }

    Ok(())
}

fn run_quiz(
    pairs: &[VocabPair],
    words: &[String],
    rng: &mut impl Rng,
    amount_incorrect_answer: usize,
) -> usize {
    let mut correct: usize = 0;

    for (idx, pair) in pairs.iter().enumerate() {
        let word = pair.get_word();
        let translate = pair.get_translate();

        let choices = make_choices(&translate, words, rng, amount_incorrect_answer);
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
    matches!(
        Confirm::new("Пройти тест заново?")
            .with_default(false)
            .prompt(),
        Ok(true)
    )
}

fn make_choices<'a>(
    right: &'a str,
    pool: &'a [String],
    rng: &mut impl Rng,
    amount_incorrect_answer: usize,
) -> Vec<&'a str> {
    let mut choices: Vec<&str> = pool
        .choose_multiple(rng, amount_incorrect_answer)
        .map(String::as_str)
        .collect();

    choices.push(right);
    choices.shuffle(rng);
    choices
}

fn choose_test_size(pairs_len: usize) -> usize {
    let options = vec![
        TestMode::Express(10),
        TestMode::Express(30),
        TestMode::Express(50),
        TestMode::Full,
    ];

    let selection = Select::new("Выберите режим теста:", options)
        .prompt()
        .unwrap_or(TestMode::Full);

    selection.size(pairs_len)
}
