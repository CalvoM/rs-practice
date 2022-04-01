use practice::q2::query::{Currency, currency_converter};
fn main(){
    let usa = Currency{name: String::from("USA"), value: 23.8};
    let mut kenya = Currency{name: String::from("KES"), value: 0.0};
    currency_converter(&usa, &mut kenya, 115.6);
    println!("From {} {} to {}{}",usa.value, usa.name, kenya.value, kenya.name)
}