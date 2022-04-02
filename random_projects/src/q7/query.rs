use crate::commons::get_user_input_uint;
use rand::Rng;
use std::io::Write;
use std::cmp::Ordering;

pub fn guessing_game(){
    let secret: u32 = rand::thread_rng().gen_range(1..101);
    let chances = 7;
    println!("Please guess a number between 1 and 100.\r\nYou have {} chances.", chances);
    let mut i =0;
    loop{
        if i == chances{
            break;
        }
        print!("Provide your guess: ");
        std::io::stdout().flush().unwrap();
        let user_input = get_user_input_uint();
        match user_input.cmp(&secret){
            Ordering::Less => println!("Ooops...Small"),
            Ordering::Greater => println!("Oh no!...Large"),
            Ordering::Equal => {
                println!("🎉 🎉 You win 🎉 🎉, the number was {}", secret);
                break;
            }
        }
        i += 1;
    }
}