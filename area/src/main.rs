#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }
    fn can_fit(&self, other:&Rectangle) -> bool{
        self.width >= other.width && self.height >= other.height
    }
    fn square(size:u32) -> Rectangle{
        Rectangle{width:size, height:size}
    }
}


fn main() {
    let rect1 = Rectangle{width:30,height:50};
    let rect2 = Rectangle{width:10,height:10};
    let square = Rectangle::square(20);
    println!("The area of the rectangles are {} and {}",rect1.area(),rect2.area());
    println!("Can the first rectangle hold the second? {}",rect1.can_fit(&rect2));
    println!("The area of our square is {}",square.area())
}
