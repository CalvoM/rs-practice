use smart_pointers::MyBox;

fn hello_world(name: &str) {
    println!("Hello, {}", name);
}
fn main() {
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    // y.drop(); cannot be called manually
    drop(y);
    let name = MyBox::new(String::from("Kazi ya Msalaba"));
    hello_world(&name);
}
