use std::{collections::HashMap, fs};

pub fn question_1() {
    let content = fs::read_to_string("./src/day_six/input.txt").unwrap();
    let content = content.trim_end();
    let mut non_marker_index = 0;
    println!("{}", content);
    for (i, _) in content.chars().enumerate() {
        let max = i + 13;
        let mut index = i;
        let mut checker: HashMap<char, usize> = HashMap::new();
        let mut no_copies = false;
        while index <= max {
            let chara = content.chars().nth(index).unwrap();
            let value = checker.get(&chara);
            if value.is_none() {
                checker.insert(chara, 1);
                no_copies = true;
            } else {
                no_copies = false;
                break;
            }
            index += 1;
        }
        if no_copies {
            non_marker_index = i + 14;
            break;
        }
    }
    println!("{}", non_marker_index);
}
