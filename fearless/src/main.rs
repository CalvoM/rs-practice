use std::thread;
use std::sync::{mpsc, Mutex};

fn main() {
    let (tx, rx) = mpsc::channel();
    let numbers = vec![1,2,3,4,5,6,7,8,9];
    let m = Mutex::new(5);
    thread::spawn(move || {
        for n in numbers {
            tx.send(n).unwrap();
        }
    });
    for recv in rx {
        println!("Received: {}", recv);
    }
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);
}
