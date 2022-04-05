use std::io::Write;
pub fn print_fibonacci(pre: u32, cur: u32, limit: u32) {
    if cur > limit {
        println!();
        return;
    }
    print!("{} ", cur);
    std::io::stdout().flush().unwrap();
    return print_fibonacci(cur, cur + pre, limit);
}
