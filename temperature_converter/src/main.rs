fn main() {
    println!("Hello, world!");
    let c = fahrenheit_to_celsius(212.0);
    println!("{}",c);
    let c = celsius_to_fahrenheit(c);
    println!("{}",c);
}

fn fahrenheit_to_celsius(temp: f64) ->f64{
    let temp = temp - 32.0;
    temp*(5.0/9.0)
}

fn celsius_to_fahrenheit(temp: f64) -> f64{
    let temp = temp * (9.0/5.0);
    temp+32.0
}
