fn get_largest<T>(list: &[T]) -> T{
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}

struct InclusivePoint<T, U> {
    x: T,
    y: U,
}
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let largest = get_largest(&number_list);
    println!("The largest number is {}", largest);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = get_largest(&char_list);
    println!("The largest of char is {}", result);
    let integer_point = Point{x: 5, y: 10};
    let float_point = Point{x: 1.0, y: 5.0};
    let not_working_point = Point{x: 5, y:6.0}; // diff types, error
    let cultured_point = InclusivePoint{x:1, y: 3.0}; //diff types, ok
}
