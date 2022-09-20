use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Orientation};
use gtk4 as gtk;

fn main() {
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let button_incr = Button::builder()
        .label("Increase")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    let button_decr = Button::builder()
        .label("Decrease")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    
    let mut number = 0;
    button_incr.connect_clicked(|_| number += 1);
    button_decr.connect_clicked(|_| number -= 1);

    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    
    gtk_box.append(&button_incr);
    gtk_box.append(&button_decr);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("GTK App")
        .child(&gtk_box)
        .build();
    window.present();
}
