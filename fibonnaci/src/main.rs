fn main() {
    println!("Hello, world!");
    println!("{}",fibonacci(7));
}

fn fibonacci(n:usize) ->usize{
    let mut cnt:usize = 1;
    let mut fib:usize =0;
    let mut pre:usize = 0;
    let mut cur:usize = 1;
    while cnt<n{
        fib = cur+pre;
        pre = cur;
        cur = fib;
        cnt = cnt+1;
    }
    pre

}