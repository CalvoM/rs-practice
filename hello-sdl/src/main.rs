use sdl2::{
    event::Event,
    image::{InitFlag, LoadTexture},
    keyboard::Keycode,
    pixels::Color,
};
use std::time::Duration;
fn main() {
    let sdl_ctx = sdl2::init().unwrap();
    let video = sdl_ctx.video().unwrap();
    let _image_ctx = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();
    let window = video.window("Demo", 800, 600).opengl().build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .load_texture("./resources/hello_world.bmp")
        .unwrap();
    canvas.copy(&texture, None, None);
    canvas.present();
    'mainloop: loop {
        for event in sdl_ctx.event_pump().unwrap().poll_iter() {
            match event {
                Event::Quit { .. } => break 'mainloop,
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => break 'mainloop,
                Event::MouseMotion { ..} => {
                    let mut mouse_state = sdl_ctx.event_pump().unwrap().mouse_state();
                    println!("Mouse coords: x = {} y = {}", mouse_state.x(), mouse_state.y());
                }
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1));
    }
}
