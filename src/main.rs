use portmanteau::load_word_list;
use portmanteau::data::PrefixBase;

// use std::collections::HashSet;

fn main() {
    let words = load_word_list().unwrap();
    println!("{:#?}", &words[0..100]);

    // let mut prefixes = HashSet::new();
    // for w in &words {
    //     prefixes.insert(w[..w.len().min(5)].to_string());
    // }
    // dbg!(prefixes.len());

    let prefixes = PrefixBase::new(&words);
    prefixes.successors("prefix", |ws| {
        for w in ws {
            println!("{}", w);
        }
    })
}
