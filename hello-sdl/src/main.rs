use sdl2::{pixels::Color, image::{InitFlag, LoadTexture}};
use std::time::Duration;
fn main() {
    let sdl_ctx = sdl2::init().unwrap();
    let video = sdl_ctx.video().unwrap();
    let _image_ctx = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();
    let window = video.window("Demo", 800, 600).opengl().build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("./resources/hello_world.bmp").unwrap();
    canvas.copy(&texture, None, None);
    canvas.present();
    ::std::thread::sleep(Duration::new(5,0));
}
