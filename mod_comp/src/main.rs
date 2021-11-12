extern crate outermost;
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::{Red, Yellow};
use outermost::inside;

fn main() {
    let _red = Red;
    let _yellow = Yellow;
    let _green = TrafficLight::Green;
    inside::inner_function();
    inside::secret_function();
}