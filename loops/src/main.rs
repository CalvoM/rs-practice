fn main() {
    let mut number = 0;
    loop{
        if number > 9{
            break;
        }
        println!("Hello, world!");
        number = number + 1;
    }
    let heights = [1.2, 1.4, 1.6, 1.8, 0.9];
    for height in heights.iter(){
        println!("Height: {}",height)
    }
    for i in (1..6).rev(){
        print!("{} ",i);
    }
}
