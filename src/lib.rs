use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use crate::search::Searcher;
use crate::data::PrefixBase;

pub mod data;
pub mod search;

pub fn load_word_list(list: &str) -> io::Result<Vec<String>> {
    let file = File::open(list)?;
    let all_words = BufReader::new(file)
        .lines()
        .collect::<io::Result<Vec<String>>>()?;
    let words = all_words
        .into_iter()
        .filter(|w| w.len() > 3)
        .collect::<Vec<_>>();
    Ok(words)
}

pub struct ListData {
    words: Vec<String>,
    prefixes: PrefixBase,
}

impl ListData {
    pub fn load_and_index(list: &str) -> io::Result<ListData> {
        let words = load_word_list(list)?;
        Ok(ListData {
            prefixes: PrefixBase::new(&words),
            words,
        })
    }

    pub fn searcher<'a>(&'a self) -> Searcher<'a> {
        Searcher::new(&self.prefixes, &self.words)
    }
}
