enum NumOrStr {
    Num(i16),
    Str(String),
}

fn main() {
    let a_number = NumOrStr::Str(String::from("Test"));
}
