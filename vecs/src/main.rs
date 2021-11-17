fn main() {
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    for i in &mut v{
        *i += 50;
        println!("{}",i);
    }
    
}
