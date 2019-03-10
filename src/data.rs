use std::collections::HashMap;

pub struct PrefixBase<'a> {
    data: HashMap<String, Vec<&'a str>>,
}

static MIN_PREFIX : usize = 2;
static MAX_PREFIX : usize = 4;

impl<'a> PrefixBase<'a> {
    pub fn new(words: &'a [String]) -> Self {
        let mut data = HashMap::new();
        for len in MIN_PREFIX..MAX_PREFIX {
            for w in words {
                if w.len() < len {
                    continue;
                }
                let entry = data.entry(w[..len].to_string()).or_insert_with(|| Vec::new());
                entry.push(&**w);
            }
        }
        PrefixBase { data }
    }

    pub fn successors(&self, s: &str, mut f: impl FnMut(&[&'a str])) {
        for len in MIN_PREFIX..MAX_PREFIX {
            let s_len = s.len();
            if len > s_len {
                return;
            }
            if let Some(words) = self.data.get(&s[s_len-len..]) {
                f(words)
            }
        }
    }
}
