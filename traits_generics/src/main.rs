fn get_largest<T>(list: &[T]) -> T{
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let largest = get_largest(&number_list);
    println!("The largest number is {}", largest);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = get_largest(&char_list);
    println!("The largest of char is {}", result);
}
