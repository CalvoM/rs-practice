fn get_largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item
        }
    }
    largest
}
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let largest = get_largest(&number_list);
    println!("The largest number is {}", largest);
    let number_list = vec![102, 34, 600, 3, 90, 188];
    let largest = get_largest(&number_list);
    println!("The largest number is {}", largest)
}
