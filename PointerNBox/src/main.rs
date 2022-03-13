use std::mem;

#[allow(dead_code)]
struct School {
    name: String,
    population: u16,
}
fn main() {
    let triza = Box::new(School {name: String::from("Triza"), population:200});
    println!("{}", mem::size_of_val(&triza));
}
