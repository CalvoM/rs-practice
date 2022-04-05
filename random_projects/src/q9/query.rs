use crate::commons::{print, PrintColour};
pub fn analyse_input_chars(input: &String) -> u32 {
    let mut vowel_cnt: u32 = 0;
    let mut consonant_cnt: u32 = 0;
    for (_, c) in input.chars().enumerate() {
        if c.is_ascii_alphabetic() {
            if is_a_vowel(&c) {
                vowel_cnt += 1;
            } else {
                consonant_cnt += 1;
            }
        } else {
            print(String::from("Early termination"), PrintColour::ERROR);
            break;
        }
    }
    println!("Vowels found: {}", vowel_cnt);
    println!("Consonants found: {}", consonant_cnt);
    vowel_cnt + consonant_cnt
}

fn is_a_vowel(c: &char) -> bool {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    if vowels.contains(c) {
        true
    } else {
        false
    }
}
