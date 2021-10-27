struct Point(i32,i32,i32);
struct Color(i32,i32,i32);

fn main() {
    let _black = Color(0,0,0);
    let origin = Point(0,0,0);
    println!("x: {}, y:{}, z:{}",origin.0,origin.1,origin.2);
}
