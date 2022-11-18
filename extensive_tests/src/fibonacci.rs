pub fn solve(index: usize) -> usize {
    let mut current: usize = 0;
    let mut later: usize = 1;
    let mut f_index: usize = 0;
    while f_index < index {
        (later, current) = (current + later, later);
        f_index += 1;
    }
    current
}
