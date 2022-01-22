//fn get_largest<T>(list: &[T]) -> T{
//    let mut largest = list[0];
//    for &item in list {
//        if item > largest {
//            largest = item;
//        }
//    }
//    largest
//}

struct Point<T> {
    x: T,
    y: T,
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32{
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
struct InclusivePoint<T, U> {
    x: T,
    y: U,
}
impl<T, U> InclusivePoint<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
    
    fn y(&self) -> &U {
        &self.y
    }
    
    fn mixup<V,W> (self, other: InclusivePoint<V, W>) -> InclusivePoint<T,W> {
        InclusivePoint{
            x: self.x,
            y: other.y,
        }
    }
}
fn main() {
//    let number_list = vec![34, 50, 25, 100, 65];
//    let largest = get_largest(&number_list);
//    println!("The largest number is {}", largest);
//    let char_list = vec!['y', 'm', 'a', 'q'];
//    let result = get_largest(&char_list);
//    println!("The largest of char is {}", result);
    let float_point = Point{x: 3.0, y: 4.0};
    println!("Distance for the origin is for floating point is {}", float_point.distance_from_origin());
    let cultured_point = InclusivePoint{x:1, y: 3.0}; //diff types, ok
    let confused_point = InclusivePoint{x: 5, y: 9.0};
    println!("p.x = {}", cultured_point.x());
    println!("p.y = {}", cultured_point.y());
    let mixed = cultured_point.mixup(confused_point);
    println!("mixed x = {}", mixed.x());
    println!("mixed y = {}", mixed.y());
}
