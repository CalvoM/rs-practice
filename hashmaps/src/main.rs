use std::collections::HashMap;

fn main() {
    let mut _scores = HashMap::new();
    _scores.insert(String::from("Blue"), 10);
    _scores.insert(String::from("Yellow"), 50);
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10,50];
    let mut scores: HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();
    for s in scores.iter() {
        println!("{} = {}",s.0,s.1);
    }
    for (k,v) in &scores {
        println!("{} = {} ",k,v);
    }
    let blue = String::from("Blue");
    let sc = 20;
    scores.insert(&blue,&sc);
    scores.entry(&blue).or_insert(&60);
    println!("{:?}", scores);
    let _blue_score = scores.get(&String::from("Blue"));
    let text = "hello from the other side";
    let mut word_map = HashMap::new();
    for word in text.split_whitespace(){
        let count = word_map.entry(word).or_insert(0);
        *count += 1
    }
    println!("{:?}", word_map)
}
