use gui::{Button, Drawable, Screen, SelectBox};
struct TextField {
    width: u32,
    height: u32,
    placeholder: String,
}

impl Drawable for TextField {
    fn draw(&self) {
        println!(
            "Drawing a textfield of width: {}, height: {} and with the placeholder: {}",
            self.width, self.height, self.placeholder
        );
    }
}
fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 75,
                options: vec![String::from("Beautiful"), String::from("Eulogy")],
            }),
            Box::new(Button {
                width: 100,
                height: 100,
                label: String::from("button1"),
            }),
            Box::new(TextField {
                width: 100,
                height: 200,
                placeholder: String::from("I am here so that I do not get fined"),
            }),
        ],
    };
    screen.run();
}
