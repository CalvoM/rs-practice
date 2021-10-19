fn main() {
    let s1 = String::from("hello");
    let len = get_length(&s1);
    println!("{} is of length {}",s1,len);
    modify_str(&s1);
    let x = 5;
    makes_copy(x);
    println!("{}",x);
}

fn makes_copy(some_integer: i32){
    println!("{}", some_integer);
}

fn get_length(s: &String) -> usize{
    s.len()
}

fn modify_str(s: &String){
    s.push_str("Test")
}