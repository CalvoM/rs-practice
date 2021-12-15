fn is_vowel(word: &String) -> bool{
    let vowels = vec!('a','e','i','o','u');
    let first_char = word.chars().next().unwrap();
    vowels.contains(&first_char)
}
fn main() {
    let mut word = String::from("apple");
    if is_vowel(&word){
        word.push_str(&String::from("-hay"));
    }else{
        println!("Consonant");
    }
    println!("{}", word);
}
