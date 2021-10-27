#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }
}

fn main() {
    let width = 30;
    let length = 50;
    let rect1 = (30, 50);
    let rect2 = Rectangle{width:30,height:50};
    println!("{} * {} = {}/{}/{}\r\nType of {:?}\r\n{:#?}",width,length,area(width, length),area_by_tuples(rect1),area_by_struct(&rect2),rect2,rect2);
    println!("{}",rect2.area());
}

fn area(w: u32, l: u32) -> u32{
    w * l
}

fn area_by_tuples(dimensions: (u32,u32)) -> u32{
    dimensions.0 * dimensions.1
}

fn area_by_struct(rect: &Rectangle) -> u32{
    rect.width * rect.height
}