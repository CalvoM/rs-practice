use std::{collections::HashMap, fs};

pub fn question_1() {
    let contents =
        fs::read_to_string("./src/day_three/input.txt").unwrap_or_else(|_| (&"").to_string());
    let contents = contents.trim();
    let mut priorities: HashMap<char, usize> = HashMap::new();
    let mut index: usize = 1;
    for c in 'a'..='z' {
        priorities.insert(c, index);
        index += 1;
    }
    for c in 'A'..='Z' {
        priorities.insert(c, index);
        index += 1;
    }
    let mut total_priorities: usize = 0;
    for rucksack in contents.split('\n') {
        let rucksack = rucksack.trim();
        let mut mid = rucksack.len() / 2;
        if rucksack.len() % 2 == 1 {
            mid += 1;
        }
        let left_sack = &rucksack[0..mid];
        let right_sack = &rucksack[mid..];
        let mut saved_items: HashMap<char, usize> = HashMap::new();
        for c in left_sack.chars() {
            saved_items.insert(c, 1);
        }
        for c in right_sack.chars() {
            let ret = saved_items.get(&c);
            if ret.is_some() {
                let priority = priorities.get(&c).unwrap();
                total_priorities += *priority;
                break;
            }
        }
    }
    println!("{}", total_priorities);
}
