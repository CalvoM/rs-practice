fn main() {
    let s = String::from("hello");
    println!("{}",s);
    let s1 = s;
    let s2 = s1.clone();
    println!("{} {}",s2,s1);
}
