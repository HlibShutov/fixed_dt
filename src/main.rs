use std::time::{Instant, Duration};
use pixels::{Pixels, SurfaceTexture};
use tao::menu::{MenuBar, MenuItem};
use tao::window::WindowBuilder;
use tao::dpi::LogicalSize;
use tao::event_loop::EventLoop;
use rand::Rng;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;

trait GameObject {
    fn update(&mut self);
    fn render(&self, frame: &mut [u8]);
}

struct PixelLoop 
{
    objects: Vec<Box<dyn GameObject>>,
    state: State,
    accumulator: Duration,
    current_time: Instant,
    time: Duration,
    dt: Duration,
    pixels: Pixels,
}

impl PixelLoop {
    fn new(state: State, update_fps: u64, pixels: Pixels) -> Self {
        Self {
            state,
            pixels,
            accumulator: Duration::new(0, 0),
            current_time: Instant::now(),
            time: Duration::default(),
            dt: Duration::from_nanos(1_000_000_000 / update_fps),
            objects: Vec::new(),
        }
    }

    fn next_loop(&mut self) {
        let new_time = Instant::now();
        let mut frame_time = new_time - self.current_time;
        self.current_time = new_time;

        if frame_time > Duration::from_millis(100) {
            frame_time = Duration::from_millis(100);
        }

        self.accumulator += frame_time;

        while self.accumulator >= self.dt {
            self.objects.iter_mut().for_each(|obj| obj.update());
            self.state.update_called += 1;
            self.accumulator -= self.dt;
            self.time += self.dt;
        }

        for (_, pixel) in self.pixels.frame_mut().chunks_exact_mut(4).enumerate() {
            let rgba = [0, 0, 0, 0];
            pixel.copy_from_slice(&rgba);
        }

        self.objects.iter_mut().for_each(|obj| obj.render(self.pixels.frame_mut()));
        let _ = self.pixels.render();
        self.state.render_called += 1;

        if self.time > Duration::from_secs(1) {
            self.display_fps();
        }
    }
    
    fn add_object(&mut self, object: Box<dyn GameObject>) {
        self.objects.push(object);
    }

    fn display_fps(&mut self) {
        println!("Update fps: {}", self.state.update_called);
        println!("render fps: {}", self.state.render_called);
        self.time = Duration::default();
        self.state.update_called = 0;
        self.state.render_called = 0;
    }

}

struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r,
            g,
            b,
            a,
        }
    }
    fn as_list(&self) -> [u8; 4]{
        [self.r, self.g, self.b, self.a]
    }
}

struct MovingBox {
    box_x: i16,
    box_y: i16,
    size: i16,
    velocity_x: i16,
    velocity_y: i16,
    color: Color,
}

impl MovingBox {
    fn new(
        box_x: i16,
        box_y: i16,
        size: i16,
        velocity_x: i16,
        velocity_y: i16,
        color: Color,
    ) -> Self {
        MovingBox{
            box_x,
            box_y,
            size,
            velocity_x,
            velocity_y,
            color,
        }
    }
}

impl GameObject for MovingBox {
    fn update(&mut self) {
        self.box_x += self.velocity_x;
        self.box_y += self.velocity_y;
        if self.box_x + self.size > WIDTH as i16 {
            self.velocity_x *= -1;
        }
        if self.box_y + self.size > HEIGHT as i16 {
            self.velocity_y *= -1;
        }
        if self.box_x + self.size < self.size {
            self.velocity_x *= -1;
        }
        if self.box_y + self.size < self.size {
            self.velocity_y *= -1;
        }
    }

    fn render(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as i16;
            let y = (i / WIDTH as usize) as i16;
            
            let inside_the_box = x >= self.box_x
                && x < self.box_x + self.size as i16
                && y >= self.box_y
                && y < self.box_y + self.size as i16;

            if inside_the_box { pixel.copy_from_slice(&self.color.as_list()) };
        }
    }
}

struct State {
    update_called: u8,
    render_called: u8,
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

    let state = State { 
        update_called: 0,
        render_called: 0,
    };

    let mut pixel_loop = PixelLoop::new(state, 120, pixels);

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
