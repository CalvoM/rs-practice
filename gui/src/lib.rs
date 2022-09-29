pub trait Drawable {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Drawable>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Drawable for Button {
    fn draw(&self) {
        println!(
            "Drawing a button of width: {}, height: {}, with the label {}",
            self.width, self.height, self.label
        );
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Drawable for SelectBox {
    fn draw(&self) {
        println!(
            "Drawing select box of width: {}, height: {} and options: {:?}",
            self.width, self.height, self.options
        );
    }
}
