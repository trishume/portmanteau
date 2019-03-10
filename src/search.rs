use crate::data::{PrefixBase, has_overlap};
use std::collections::{VecDeque, HashMap};
use std::collections::hash_map::Entry;

#[derive(Debug)]
pub enum SearchError {
    NotImplemented,
    NoPath,
}

#[derive(Debug)]
pub struct Path {
    words: Vec<String>,
}

impl Path {
    pub fn prefixes(&self) -> Vec<usize> {
        let mut res = Vec::with_capacity(self.words.len());
        for i in 0..self.words.len() {
            if i+1 < self.words.len() {
                let len = self.words[i].len();
                let next: &str = &self.words[i+1];
                for pref in 0..len {
                    if next.starts_with(&self.words[i][pref..]) {
                        res.push(pref);
                        break;
                    }
                }
            } else {
                res.push(self.words[i].len());
            }
        }
        res
    }

    pub fn joined(&self) -> String {
        self.join(&self.prefixes())
    }

    pub fn fancy_ouput(&self) -> String {
        let prefixes = self.prefixes();
        let mut s = self.join(&prefixes);

        let mut offset = 0;
        for (i,w) in self.words.iter().enumerate() {
            s.push('\n');
            for _ in 0..offset {
                s.push(' ');
            }
            s.push_str(w);
            offset += prefixes[i];
        }
        s
    }

    fn join(&self, prefixes: &[usize]) -> String {
        let mut s = String::new();
        for i in 0..self.words.len() {
            s.push_str(&self.words[i][..prefixes[i]]);
        }
        s
    }
}

pub struct Searcher<'a> {
    prefixes: &'a PrefixBase,
    words: &'a [String],

    q: VecDeque<usize>,
    preds: HashMap<usize, Option<usize>>,
}

impl<'a> Searcher<'a> {
    pub fn new(prefixes: &'a PrefixBase, words: &'a [String]) -> Self {
        Searcher {
            prefixes, words,
            q: VecDeque::new(),
            preds: HashMap::new(),
        }
    }

    pub fn search(&mut self, start: &str, target: &str) -> Result<Path,SearchError> {
        self.push_succs(start, None);

        let final_i = loop {
            let i = if let Some(i) = self.q.pop_front() {
                i
            } else {
                return Err(SearchError::NoPath)
            };

            let word = &self.words[i];
            if has_overlap(word, target) {
                break i;
            }

            self.push_succs(word, Some(i));
        };

        let mut path = vec![target.to_string()];
        let mut cur_i = final_i;
        loop {
            path.push(self.words[cur_i].to_string());
            cur_i = if let Some(Some(pred)) = self.preds.get(&cur_i) {
                *pred
            } else {
                break;
            };
        }
        path.push(start.to_string());
        path.reverse();

        Ok(Path {words: path})
    }

    fn push_succs(&mut self, s: &str, i: Option<usize>) {
        self.prefixes.successors(s, |wi| {
            if let Entry::Vacant(v) = self.preds.entry(wi) {
                v.insert(i);
                self.q.push_back(wi);
            }
        });
    }
}
