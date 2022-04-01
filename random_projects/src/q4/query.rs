pub fn print_pattern(bottom_number: u32) {
    let mut i = 1;
    loop{
        if i == bottom_number +1{
            break;
        }
        println!("{}", "*".repeat(i as usize));
        i += 1;
    }
}