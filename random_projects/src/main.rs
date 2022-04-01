use practice::q1::query::factorial;
use practice::commons::get_user_input_uint;
fn main(){
    let num = get_user_input_uint();
    let res = factorial(num);
    println!("Factorial of {} is {}", num, res)
}