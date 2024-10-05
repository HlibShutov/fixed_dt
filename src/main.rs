use pixels::{Pixels, SurfaceTexture};
use tao::menu::{MenuBar, MenuItem};
use tao::window::WindowBuilder;
use tao::dpi::LogicalSize;
use tao::event_loop::EventLoop;
use rand::Rng;

mod moving_box;
mod pixel_loop;
mod color;

use moving_box::MovingBox;
use pixel_loop::PixelLoop;
use color::Color;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;

trait GameObject {
    fn update(&mut self);
    fn render(&self, frame: &mut [u8]);
}

fn random_in_range(range: std::ops::Range<i16>) -> i16 {
    rand::thread_rng().gen_range(range)
}

fn main() {
    let event_loop = EventLoop::new();
    let window = {
        let mut file_menu = MenuBar::new();
        file_menu.add_native_item(MenuItem::Quit);

        let mut menu = MenuBar::new();
        menu.add_submenu("File", true, file_menu);

        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels/Tao")
            .with_menu(menu)
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };
    let pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
    };


    let mut pixel_loop = PixelLoop::new(120, pixels);

    for _ in 0..10 {

        let moving_box = MovingBox::new(
            random_in_range(0..270),
            random_in_range(0..290),
            random_in_range(20..50),
            random_in_range(-2..2),
            random_in_range(-2..2),
            Color::new(random_in_range(0..255) as u8, random_in_range(0..255) as u8, random_in_range(0..255) as u8, 255),
        );
        pixel_loop.add_object(Box::new(moving_box));
    }

    loop {
        pixel_loop.next_loop();
    }
}
