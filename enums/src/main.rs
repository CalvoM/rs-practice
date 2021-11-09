enum Coin{
    _Penny,
    _Nickel,
    _Dime,
    Quarter,
}

fn get_cents(coin: Coin) -> u32{
    match coin{
        Coin::_Penny => 1,
        Coin::_Nickel => 5,
        Coin::_Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let my_coin = Coin::Quarter;
    println!("Number of cents for coin = {}", get_cents(my_coin));
}
