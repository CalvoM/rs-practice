struct User{
    username: String,
    email: String, 
    sign_in_count: u64,
    active: bool,
}
fn main() {
    let user1 = User{
        email: String::from("test@gmail.com"),
        username: String::from("test"),
        active: true,
        sign_in_count: 1,
    };
    println!("{}",user1.email);
    let user2 = User{
        email: String::from("test2@gmail.com"),
        username: String::from("test2"),
        ..user1
    };
}

fn build_user(email: String, username: String) -> User{
    User{
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}