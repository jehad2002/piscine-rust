use std::collections::HashMap;
use regex::Regex;

pub fn counting_words(words: &str) -> HashMap<String, u32> {
    let re = Regex::new(r"\b[\w]+(?:'[a-zA-Z]+)?\b").unwrap();

    let mut counts = HashMap::new();

    for mat in re.find_iter(words) {
        let word = mat.as_str().to_lowercase();
        *counts.entry(word).or_insert(0) += 1;
    }

    counts
}

fn main() {
    println!("{:?}", counting_words("Hello, world!"));
    println!("{:?}", counting_words("“Two things are infinite: the universe and human stupidity; and I'm not sure about the universe.” ― Albert Einstein "));
    println!("{:?}", counting_words("Batman, BATMAN, batman, Stop stop"));
}
