use std::{thread, time::Duration};

type CallableOnceString = Option<Box<dyn Fn(String)>>;
struct Alert<'a> {
    message: String,
    msg_handler: &'a CallableOnceString,
}
impl<'a> Alert<'a> {
    fn new() -> AlertBuilder {
        AlertBuilder {
            message: String::new(),
            msg_handler: None,
        }
    }
    fn run(&self) {
        let mut i = 0;
        loop {
            thread::sleep(Duration::from_millis(100));
            if i == 10 {
                if let Some(f) = self.msg_handler {
                    f(self.message.clone());
                }
                break;
            }
            i += 1;
        }
    }
}

struct AlertBuilder {
    message: String,
    msg_handler: CallableOnceString,
}

impl AlertBuilder {
    fn message(&mut self, message: String) -> &mut Self {
        self.message = message;
        self
    }
    fn on_message(&mut self, msg_handler: CallableOnceString) -> &mut Self {
        self.msg_handler = msg_handler;
        self
    }
    fn build(&mut self) -> Alert {
        Alert {
            message: self.message.clone(),
            msg_handler: &self.msg_handler,
        }
    }
}

fn main() {
    println!("Hello, world!");
    let msg_handler = |msg: String| {
        println!("Received the alert: `{}`", msg);
    };
    let mut binding = Alert::new();
    let alert = binding
        .message("Njaa".to_owned())
        .on_message(Some(Box::new(msg_handler)))
        .build();
    alert.run();
}
