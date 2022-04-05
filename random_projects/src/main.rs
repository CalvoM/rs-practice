use practice::q9::query::analyse_input_chars;
use practice::commons::{get_user_input_str, stdout_fflush};
use std::convert::TryFrom;
fn main(){
    print!("Enter the string to analyze: ");
    stdout_fflush();
    let input: String = get_user_input_str().trim().to_string();
    let c = analyse_input_chars(&input);
    if input.len() > usize::try_from(c).unwrap() {
        println!("Did not parse the full string provided, encountered a non-alphabet character.");
    }
}