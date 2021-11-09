fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    let def_number: i8 = 5;
    let sum = some_number + def_number;
    println!("{}", sum);
}
