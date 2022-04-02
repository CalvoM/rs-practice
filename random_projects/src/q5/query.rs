pub fn print_pattern(bottom_number: u32) {
    let mut i: u32 = 1;
    if bottom_number%2 == 0{
        i = 2
    }

    loop{
        if i > bottom_number {
            break;
        }
        println!("{}", "*".repeat(i as usize));
        i += 2;
    }
}