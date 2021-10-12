extern crate rand;

use rand::Rng;

fn main() {
    let rand_num = rand::thread_rng().gen_range(1..11);
    let remark = if rand_num < 5{
        "Less than 5"
    }else{
        "More than 5"
    };
    println!("{}",remark);
}
