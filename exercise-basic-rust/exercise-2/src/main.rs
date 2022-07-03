use regex::{escape, Regex};
use std::io::stdin;

fn main() {
    let text = "This is a regular para&graph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal.";
    let mut word = String::new();
    println!("Please enter a word:");
    stdin().read_line(&mut word).unwrap();
    println!("======================");
    let pattern = format!(r"(?i){}", word.trim());
    println!("Pattern: \"{}\"", &pattern);
    let re = Regex::new(&pattern.as_str()).unwrap();
    let word_cound = re.captures_iter(text).count();
    println!("Word cound: {}", word_cound);
}
