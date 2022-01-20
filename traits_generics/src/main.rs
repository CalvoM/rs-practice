fn get_largest_num(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item
        }
    }
    largest
}

fn get_largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for & item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let largest = get_largest_num(&number_list);
    println!("The largest number is {}", largest);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = get_largest_char(&char_list);
    println!("The largest of char is {}", result);
}
