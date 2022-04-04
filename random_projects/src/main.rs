use practice::q9::query::analyse_input_chars;
use practice::commons::{get_user_input_str, stdout_fflush};
fn main(){
    print!("Enter the string to analyze: ");
    stdout_fflush();
    let input: String = get_user_input_str().trim().to_string();
    let c = analyse_input_chars(input);
    println!("{}", c)
}