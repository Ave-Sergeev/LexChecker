use std::error::Error;
use crate::lexicon::dictionary::Dictionary;
use inquire::Select;
use rand::seq::{IndexedRandom, SliceRandom};
use std::path::Path;
use std::str::FromStr;
use env_logger::Builder;
use log::LevelFilter;
use crate::lexicon::words::WordsPool;
use crate::setting::settings::Settings;

mod lexicon;
mod setting;

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::rng();
    let mut correct: usize = 0;

    let settings = Settings::new("config.yaml", "APP").map_err(|err| format!("Failed to load setting: {err}"))?;

    Builder::new()
        .filter_level(LevelFilter::from_str(settings.logging.log_level.as_str()).unwrap_or(LevelFilter::Info))
        .init();

    log::info!("Settings:\n{}", settings.json_pretty());

    let words_path = Path::new(&settings.app.words_path);
    let dictionary_path = Path::new(&settings.app.dictionary_path);

    let words_pool = WordsPool::new(words_path);
    let dictionary = Dictionary::new(dictionary_path);

    let words = words_pool.get_words().unwrap();
    let mut pairs = dictionary.get_pairs();
    pairs.shuffle(&mut rng);

    if pairs.is_empty() || words.is_empty() {
        log::error!("File lexicon.txt or words.txt is empty");
        return Ok(());
    }

    for (idx, pair) in pairs.iter().enumerate() {
        let word = pair.get_word();
        let translate = pair.get_translate();

        let choices = make_choices(&translate, &words, &mut rng);
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

    println!("Результат: {}/{} слов.", correct, pairs.len());

    Ok(())
}

fn make_choices<'a>(right: &'a str, pool: &'a [String], rng: &mut impl rand::Rng) -> Vec<&'a str> {
    let mut choices: Vec<&str> = pool
        .choose_multiple(rng, 3)
        .map(String::as_str)
        .collect();

    choices.push(right);
    choices.shuffle(rng);
    choices
}
