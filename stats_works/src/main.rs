//Given a list of integers, use a vector and return the mean (the average
//value), median (when sorted, the value in the middle position), and
//mode (the value that occurs most often; a hash map will be helpful here)
//of the list.
use std::collections::HashMap;
fn main() {
    let mut scores = vec![10, 1, 9, 2, 5, 1, 3, 3, 6, 7, 4, 8, 30, 21, 20, 15, 15,1,20];
    println!("Calculating the mean for the supplied numbers {:?}", scores);
    let vec_size = scores.len() as f32;
    let vec_sum: u16 = scores.iter().sum();
    let average: f32 = f32::from(vec_sum as f32 / vec_size);
    println!("Average: {:.4}", average);
    println!("Calculating the mode for the supplied number {:?}", scores);
    let mut number_count = HashMap::new();
    for s in &scores{
        let count = &mut number_count.entry(s).or_insert(0);
        **count += 1;
    }
    let mut modal_value: u16 = 0;
    let mut modal_key:u16 = 0;
    for (number, freq) in number_count.iter(){
        if freq > &modal_value {
            modal_value = *freq as u16;
            modal_key = **number as u16;
        }
    }
    println!("Modal number: {}",modal_key);
    scores.sort();
    println!("Calculating the median for the supplied numbers {:?} of size {}", scores, scores.len());
    let scores_size = scores.len();
    let mut median_num = 0.0;
    if scores_size%2 == 0{
        median_num = ((scores[scores_size/2]-1) as f32 + scores[(scores_size/2)] as f32)/2.0;
    }else{
        median_num = scores[scores_size/2] as f32;
    }
    println!("Median number: {}", median_num);
}
