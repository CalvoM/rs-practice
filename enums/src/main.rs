enum IpAddr{
    V4(u8,u8,u8,u8),
    V6(String),
}


enum Message{ //equivalent struct as commented below
    Quit, //Struct QuitMessage;
    Move {x: i32, y: i32}, //Struct MoveMessage{x: i32, y: i32};
    Write(String), //Struct WriteMessage(String);
    ChangeColor(i32, i32, i32), //Struct ChangeColorMessage(i32, i32, i32);
}

impl Message{
    fn call(&self){
        println!("{}", "What");
    }
}

fn main() {
    let _home = IpAddr::V4(127,0,0,1);
    let _loopback = IpAddr::V6(String::from("::1"));
    let m = Message::Write(String::from("Hello"));
    println!("{}", m.call());
}
