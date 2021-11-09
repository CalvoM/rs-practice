fn main() {
    let some_u8_value = Some(8);
    if let Some(3) = some_u8_value{
        println!("three")
    }else{
        println!("not three")
    }
}
