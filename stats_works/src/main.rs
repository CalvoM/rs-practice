//Given a list of integers, use a vector and return the mean (the average
//value), median (when sorted, the value in the middle position), and
//mode (the value that occurs most often; a hash map will be helpful here)
//of the list.
fn main() {
    let mut scores = vec![10, 1, 9, 2, 5, 1, 3, 3, 6, 7, 4, 8, 30, 21, 20, 15, 15];
    println!("Calculating the mean for the supplied numbers {:?}", scores);
    let vec_size = scores.len() as f32;
    let vec_sum: u16 = scores.iter().sum();
    let average: f32 = f32::from(vec_sum as f32 / vec_size);
    println!("Average: {:.4}", average)
}
