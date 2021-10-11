fn main() {
    println!("Hello, world!");
    another_fxn();
    py_calc(3, 4);
    let five = dummy_func();
    println!("{}",five)
}

fn another_fxn(){
    println!("from another function!");
}

fn py_calc(a: i32, b:i32){
    let c = a*a + b*b;
    println!("The square of the hypotenuse is {}",c);
}

fn dummy_func() -> i32{
    5
}