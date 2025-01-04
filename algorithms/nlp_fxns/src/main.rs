use std::collections::{hash_map::Entry, HashMap,};

use regex::Regex;

fn main() {
    println!("Hello, world!");
    // Working with Strings
    let s1 = String::from("NLP is cool, it has several applications");
    let s2: &str = "This is a string slice";
    // Join
    println!("{s1}{s2}");

    // Text Normalization
    // Convert to lowercase
    println!("{}",s1.to_lowercase());
    println!("{}",s1.to_uppercase());

    // Tokenize
    let tokens = tokenize_text(&s1);
    println!("{:?}",tokens);
    let tokens2 = tokenize_text_with_regex(&s1);
    println!("{:?}",tokens2);

    let tokens3 = tokenize_by_characters(&s1);
    println!("{:?}",tokens3);

    // Word Count
    println!("{:?}",word_count(&s1));

}

fn tokenize_text(text: &str) -> Vec<String> {
    text.to_lowercase().split_whitespace().map(String::from).collect()
}

fn tokenize_text_with_regex(text: &str) -> Vec<&str>{
    let re = Regex::new(r"\w+").unwrap();
    re.find_iter(text).map(|m|m.as_str()).collect()
}

fn tokenize_by_characters(text: &str)-> Vec<char>{
    text.chars().collect()
}

// dictionary to store
// tokenize the text
// loop through if it is found in the dictionary you add +1 else 1


fn word_count(text: &str) -> HashMap<String,usize>{
    let mut word_counts: HashMap<String,usize> = HashMap::new();
    let tokens: Vec<String>= text.to_lowercase().split_whitespace().map(String::from).collect();
    for word in tokens{
        match word_counts.entry(word){
            Entry::Vacant(e) => {e.insert(1);},
            Entry::Occupied(mut e) => {*e.get_mut() +=1;},
        }

    }
    word_counts

}