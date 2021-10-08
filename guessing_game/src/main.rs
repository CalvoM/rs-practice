extern crate rand;

use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
    loop{
        let mut guess = String::new();
        println!("Guess the number: ");
        io::stdin().read_line(&mut guess)
            .expect("Failed to read the numbers");
        println!("You guessed: {}",guess);
        let guess:u32 = guess.trim().parse()
            .expect("Please type a number!");
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
            println!("You win!");
            break;
        }
        }

    }
}
