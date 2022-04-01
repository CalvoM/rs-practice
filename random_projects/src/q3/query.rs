pub fn print_pattern(top_number: u32) {
    let mut i = top_number;
    loop{
        if i == 0 {
            break;
        }
        println!("{}", "*".repeat(i as usize));
        i -= 1;
    }
}