use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

pub mod data;
pub mod search;

pub fn load_word_list() -> io::Result<Vec<String>> {
    let file = File::open("data/google-10000-english-usa-no-swears.txt")?;
    let all_words = BufReader::new(file)
        .lines()
        .collect::<io::Result<Vec<String>>>()?;
    let words = all_words
        .into_iter()
        .filter(|w| w.len() > 2)
        .collect::<Vec<_>>();
    Ok(words)
}
