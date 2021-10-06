use std::io;

fn main() {
    println!("Guess the number: ");
    println!("Hit me: ");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read the number");
    println!("You guessed: {}",guess);
}
