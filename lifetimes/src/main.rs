fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        let e = x;
        e
    } else {
        y
    }
}
fn main() {
    let s1 = String::from("abcd");
    let res;
    {
        let s2 = String::from("xyz");
        res = longest(s1.as_str(), s2.as_str());
    }
        println!("The longest string is {}", res);
}
