use crate::commons::{print,PrintColour};
pub fn word_scrambler(word: &String) -> Vec<String> {
    let mut scrambled_words: Vec<String> = Vec::new();
    let scrambled_words_c = word.chars();
    for (i, c) in scrambled_words_c.enumerate() {
        print(format!("{}, {}",i,c), PrintColour::INFO);
    }
    scrambled_words
}