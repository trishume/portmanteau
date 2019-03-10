use std::collections::HashMap;

pub struct PrefixBase {
    data: HashMap<String, Vec<usize>>,
}

static MIN_PREFIX : usize = 2;
static MAX_PREFIX : usize = 5;

impl PrefixBase {
    pub fn new(words: &[String]) -> Self {
        let mut data = HashMap::new();
        for len in MIN_PREFIX..MAX_PREFIX {
            for (i,w) in words.iter().enumerate() {
                if w.len() < len {
                    continue;
                }
                let entry = data.entry(w[..len].to_string()).or_insert_with(|| Vec::new());
                entry.push(i);
            }
        }
        PrefixBase { data }
    }

    pub fn successors(&self, s: &str, mut f: impl FnMut(usize)) {
        for len in MIN_PREFIX..MAX_PREFIX {
            let s_len = s.len();
            if len > s_len {
                return;
            }
            if let Some(words) = self.data.get(&s[s_len-len..]) {
                for w in words.iter().cloned() {
                    f(w)
                }
            }
        }
    }
}

pub fn has_overlap(a: &str, b: &str) -> bool {
    for len in MIN_PREFIX..MAX_PREFIX {
        if len > a.len() || len > b.len() {
            break;
        }
        if a[a.len()-len..] == b[..len] {
            return true;
        }
    }
    return false;
}
