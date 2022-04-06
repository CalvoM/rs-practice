use practice::q11::query::word_scrambler;
use practice::commons::{get_user_input_str, stdout_fflush};
fn main(){
    print!("Enter the string to scramble: ");
    stdout_fflush();
    let input: String = get_user_input_str().trim().to_string();
    let c = word_scrambler(&input);
    println!("{:?}", c);
}