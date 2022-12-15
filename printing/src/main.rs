#![allow(overflowing_literals)]
use std::fmt;
#[derive(Debug)]
struct Complex {
    r: f64,
    i: f64,
    t: Vec<f64>,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}+{}i", self.r, self.i)
    }
}

enum Temperature {
    Celsius(i32),
    Farenheit(i32),
}

fn main() {
    let origin = Complex {
        r: 8 as f64,
        i: 10 as f64,
        t: vec![1.0, 2.0],
    };
    let temp = Temperature::Celsius(15);
    match temp {
        Temperature::Celsius(t @ 20..) => println!("{}C is above 30 Celsius", t),
        Temperature::Celsius(t) => println!("{}C is below 30 Celsius", t),
        Temperature::Farenheit(t) if t > 86 => println!("{}F is above 86 F", t),
        Temperature::Farenheit(t) => println!("{}F is below 86F", t),
    }
}
