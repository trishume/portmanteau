use portmanteau::ListData;

// use std::collections::HashSet;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: portmanteau START END");
        return;
    }

    let small = ListData::load_and_index("data/google-10000-english-usa-no-swears.txt").unwrap();
    let large = ListData::load_and_index("data/wordlist.asc.txt").unwrap();

    let (a,b) = (&args[1], &args[2]);
    let res = small.searcher().search(a,b).or_else(|_| {
        large.searcher().search(a, b)
    });
    match res {
        Ok(path) => {
            eprintln!("Path found:\n");
            println!("{}", path.fancy_ouput())
        }
        Err(_) => println!("Path not found!  :("),
    }
}
