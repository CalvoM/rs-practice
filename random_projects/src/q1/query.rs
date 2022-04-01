
pub fn factorial(num: u32) -> u32 {
    if num < 1 {
        1
    } else {
        num * factorial(num -1)
    }
}