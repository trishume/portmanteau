use crate::data::{PrefixBase, has_overlap, MAX_PREFIX};
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;
use std::usize;

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

#[derive(PartialEq, Eq)]
struct InvertOrder(usize);

impl PartialOrd for InvertOrder {
    fn partial_cmp(&self, other: &InvertOrder) -> Option<Ordering> {
        Some(other.0.cmp(&self.0))
    }
}

impl Ord for InvertOrder {
    fn cmp(&self, other: &InvertOrder) -> Ordering {
        other.0.cmp(&self.0)
    }
}

pub struct Searcher<'a> {
    prefixes: &'a PrefixBase,
    words: &'a [String],

    q: BinaryHeap<(InvertOrder, usize)>,
    preds: HashMap<usize, Option<usize>>,
    dists: Vec<usize>,
}

impl<'a> Searcher<'a> {
    pub fn new(prefixes: &'a PrefixBase, words: &'a [String]) -> Self {
        Searcher {
            q: BinaryHeap::new(),
            preds: HashMap::new(),
            dists: (0..words.len()).map(|_| usize::MAX).collect(),
            prefixes, words,
        }
    }

    pub fn search(&mut self, start: &str, target: &str) -> Result<Path,SearchError> {
        self.push_succs(start, 0, None);

        let final_i = loop {
            let (InvertOrder(d),i) = if let Some((d,i)) = self.q.pop() {
                (d,i)
            } else {
                return Err(SearchError::NoPath)
            };

            // we may have found a better way here earlier
            if d > self.dists[i] { continue; }

            let word = &self.words[i];
            if has_overlap(word, target) {
                break i;
            }

            self.push_succs(word, d, Some(i));
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

    fn push_succs(&mut self, s: &str, base_dist: usize, i: Option<usize>) {
        self.prefixes.successors(s, |wi, len| {
            // let weight = 1 + MAX_PREFIX - len;
            let weight = match len {
                1 => 100,
                2 => 10,
                3 => 2,
                _ => 1
            };
            let next_dist = base_dist + weight;
            if next_dist < self.dists[wi] {
                self.dists[wi] = next_dist;
                self.preds.insert(wi, i);
                self.q.push((InvertOrder(next_dist), wi));
            }
        });
    }
}
