use practice::q8::query::print_fibonacci;
use practice::commons::get_user_input_uint;
use std::io::Write;
fn main(){
    print!("Provide limit for the fibonacci series: ");
    std::io::stdout().flush().unwrap();
    let limit = get_user_input_uint();
    print_fibonacci(0,1,limit);
}