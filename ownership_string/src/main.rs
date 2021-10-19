fn main() {
    let mut s1 = String::from("hello");
    let len = get_length(&s1);
    println!("{} is of length {}",s1,len);
    modify_str(&mut s1);
    let len = get_length(&s1);
    println!("{} is of length {}",s1,len);
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

fn modify_str(s: &mut String){
    s.push_str("Test")
}