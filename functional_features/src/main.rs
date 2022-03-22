use std::thread;
use std::time::Duration;

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_calculation = |num: u32| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    if intensity < 25 {
        println!(
            "Today, do {} pushups",
            expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Hydrate!!!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_calculation(intensity)
            );
        }
    }
}

fn main() {
    println!("Hello, world!");
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);
}
