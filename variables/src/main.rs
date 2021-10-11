
fn main() {
    println!("This is the short story of variable x");
    let x = 5;
    println!("X was initially {}",x);
    let x = x + 1;
    println!("We added 1 and it became {}",x);
    let x = x * 2;
    println!("We multiplied by 2 to become {}",x);
    println!("What happened next will surprise you!");
    let x = "I am a string";
    println!("x decided to change its type: x said {}",x);
    let number_one = 56u8;
    println!("number one is {}",number_one);
    let long_num = 45_111;
    println!("long num is {}",long_num);
    let yen = '\u{00a5}';
    println!("Wanna see the Yen sign, oh I mean {}",yen);
    let coords: (f32, f32, f32) = (0.0,0.0,0.0);
    println!("The coordinates for origin is {} {} {}",coords.0,coords.1,coords.2);
    choose_me()
}

fn choose_me(){
    println!("I was just saying you choose me \r\nThanks for choosing me.")
}