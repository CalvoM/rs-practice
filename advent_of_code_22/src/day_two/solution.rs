use std::{collections::HashMap, fs};
pub fn question_1() {
    let ops_values: HashMap<&str, u32> = HashMap::from([("A", 1), ("B", 2), ("C", 3)]);
    let my_values: HashMap<&str, u32> = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);
    let mut counter_values: HashMap<u32, u32> = HashMap::new();
    counter_values.insert(1, 3);
    counter_values.insert(2, 1);
    counter_values.insert(3, 2);
    let contents = fs::read_to_string("./src/day_two/input.txt").expect("Reading not okay");
    let results = contents.trim().split("\n");
    let mut total_score: u64 = 0;
    for result in results {
        let values: Vec<&str> = result.split_whitespace().collect();
        let opponent = ops_values.get(values[0]).unwrap_or(&0);
        let me = my_values.get(values[1]).unwrap_or(&0);
        let enemy = counter_values.get(me).unwrap_or(&0);
        if me == opponent {
            total_score += 3 + (*me as u64);
        } else if opponent == enemy {
            total_score += 6 + (*me as u64);
        } else {
            total_score += *me as u64;
        }
    }
    println!("total score = {}", total_score);
}
