use std::fs;

pub fn question_1() {
    let contents = fs::read_to_string("./src/day_one/input1.txt").expect("Reading not okay");
    let segments = contents.split("\n\n");
    let mut max_sum: u64 = 0;
    for segment in segments {
        let numbers = segment.split("\n");
        let mut sum: u64 = 0;
        for number in numbers {
            if number.len() >= 1 {
                sum += number.parse::<u64>().unwrap();
            }
        }
        if sum >= max_sum {
            max_sum = sum;
        }
    }
    println!("The max calories is {}", max_sum);
}

pub fn question_2() {
    let contents = fs::read_to_string("./src/day_one/input1.txt").expect("Reading not okay");
    let segments = contents.split("\n\n");
    let mut max_sum: u64 = 0;
    let mut max_list: Vec<u64> = vec![];
    for segment in segments {
        let numbers = segment.split("\n");
        let mut sum: u64 = 0;
        for number in numbers {
            if number.len() >= 1 {
                sum += number.parse::<u64>().unwrap();
            }
        }
        max_list.push(sum);
    }
    max_list.sort();
    let max_three_sum: u64 = max_list.iter().rev().take(3).sum();
    println!("{:?}", max_three_sum);
}
