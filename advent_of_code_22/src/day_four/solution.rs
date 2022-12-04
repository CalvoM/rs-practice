use std::fs;

pub fn question_1() {
    let content = fs::read_to_string("./src/day_four/input.txt").unwrap();
    let content = content.trim();
    let mut total_pairs = 0;
    for line in content.split('\n') {
        let ranges = line.split(',').collect::<Vec<&str>>();
        let r0 = ranges[0].split('-').collect::<Vec<&str>>();
        let range_one = r0[0].parse::<usize>().unwrap()..(r0[1].parse::<usize>().unwrap());
        let r1 = ranges[1].split('-').collect::<Vec<&str>>();
        let range_two = r1[0].parse::<usize>().unwrap()..(r1[1].parse::<usize>().unwrap());
        match range_one.start.cmp(&range_two.start) {
            std::cmp::Ordering::Less => {
                if range_one.end >= range_two.end {
                    total_pairs += 1;
                }
            }
            std::cmp::Ordering::Greater => {
                if range_two.end >= range_one.end {
                    total_pairs += 1;
                }
            }
            std::cmp::Ordering::Equal => total_pairs += 1,
        }
    }
    println!("{}", total_pairs);
}

pub fn question_2() {
    let content = fs::read_to_string("./src/day_four/input.txt").unwrap();
    let content = content.trim();
    let mut total_pairs = 0;
    for line in content.split('\n') {
        let ranges = line.split(',').collect::<Vec<&str>>();
        let r0 = ranges[0].split('-').collect::<Vec<&str>>();
        let range_one = r0[0].parse::<usize>().unwrap()..(r0[1].parse::<usize>().unwrap());
        let r1 = ranges[1].split('-').collect::<Vec<&str>>();
        let range_two = r1[0].parse::<usize>().unwrap()..(r1[1].parse::<usize>().unwrap());
        match range_one.start.cmp(&range_two.start) {
            std::cmp::Ordering::Less => {
                if range_one.end >= range_two.start {
                    total_pairs += 1;
                }
            }
            std::cmp::Ordering::Greater => {
                if range_two.end >= range_one.start {
                    total_pairs += 1;
                }
            }
            std::cmp::Ordering::Equal => total_pairs += 1,
        }
    }
    println!("{}", total_pairs);
}
