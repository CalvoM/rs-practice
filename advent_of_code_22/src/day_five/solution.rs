use regex::Regex;
use std::{collections::HashMap, fs};

pub fn question_1() {
    enum State {
        ReadingStack,
        ReadingStackIndices,
        ProcessStack,
        ReadingCmds,
    }
    let contents = fs::read_to_string("./src/day_five/input.txt").unwrap();
    let contents = contents.trim_end();
    let lines = contents.split('\n').collect::<Vec<&str>>();
    let line_count = lines.len() - 1;
    let mut index = 0;
    let mut state = State::ReadingStack;
    let mut stacks: Vec<&str> = vec![];
    let mut stack_indices: &str = "";
    let mut stack_layout: HashMap<char, Vec<char>> = HashMap::new();
    let mut max_stack_index = 0;
    let re = Regex::new(r"move (?P<count>\d{1,2}) from (?P<src>\d+) to (?P<dest>\d+)").unwrap();
    while index <= line_count {
        match state {
            State::ReadingStack => {
                let line = lines[index];
                let first_letter = line.chars().nth(1).unwrap_or(' ');
                if first_letter != '1' {
                    stacks.push(line);
                    index += 1;
                } else {
                    state = State::ReadingStackIndices;
                }
            }
            State::ReadingStackIndices => {
                stack_indices = lines[index];
                index += 1;
                let lean = stack_indices.trim();
                let max = lean.chars().nth(lean.len() - 1).unwrap_or('0');
                max_stack_index = max.to_string().parse().unwrap();
                state = State::ProcessStack;
            }
            State::ProcessStack => {
                for (i, si) in stack_indices.chars().enumerate() {
                    if si != ' ' {
                        let mut cur_stack = stack_layout.get_mut(&si);
                        if cur_stack.is_none() {
                            let mut values_at_index: Vec<char> = vec![];
                            for stack_value in stacks.clone() {
                                let item = stack_value.chars().nth(i).unwrap();
                                if item != ' ' {
                                    values_at_index.push(stack_value.chars().nth(i).unwrap());
                                }
                            }
                            stack_layout.insert(si, values_at_index.clone());
                        }
                    }
                }
                index += 1;
                state = State::ReadingCmds;
            }
            State::ReadingCmds => {
                let cmd = lines[index];
                let caps = re.captures(cmd).unwrap();
                let count: usize = *(&caps["count"].parse().unwrap());
                let src = caps["src"].chars().nth(0).unwrap();
                let dest = caps["dest"].chars().nth(0).unwrap();
                let dest_stack = stack_layout.get(&dest).unwrap();
                let mut dest_stack_clone = dest_stack.clone();
                let src_stack = stack_layout.get(&src).unwrap();
                let mut src_stack_clone = src_stack.clone();
                let mut values: Vec<char> = src_stack_clone.drain(0..count).collect();
                // values.reverse();
                values.append(&mut dest_stack_clone);
                stack_layout.insert(src, src_stack_clone);
                stack_layout.insert(dest, values);
                index += 1;
            }
        }
    }
    for n in '1'..'9' {
        print!("{}", stack_layout.get(&n).unwrap()[0]);
    }
    println!("{:?}", stack_layout[&'9']);
}
