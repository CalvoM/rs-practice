fn main() {
    let s1 = String::from("hello");
    let (s2,len) = get_length_and_return(s1);
    println!("{} is of length {}",s2,len);
    let x = 5;
    makes_copy(x);
    println!("{}",x);
}

fn takes_ownership(some_string: String){
    println!("{}",some_string);
}

fn makes_copy(some_integer: i32){
    println!("{}", some_integer);
}

fn get_length_and_return(s: String) ->(String, usize){
    let s_len = s.len();
    (s,s_len)
}
