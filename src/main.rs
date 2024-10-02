use std::time::{Instant, Duration};
use pixels::{Pixels, SurfaceTexture};
use tao::menu::{MenuBar, MenuItem};
use tao::window::WindowBuilder;
use tao::dpi::LogicalSize;
use tao::event_loop::EventLoop;
use tao::event::Event;
use std::cell::RefCell;
use std::rc::Rc;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;

struct MovingBox {
    box_x: i16,
    box_y: i16,
    size: i16,
    velocity_x: i16,
    velocity_y: i16,
}

impl MovingBox {
    fn new(
        box_x: i16,
        box_y: i16,
        size: i16,
        velocity_x: i16,
        velocity_y: i16
    ) -> Self {
        MovingBox{
            box_x,
            box_y,
            size,
            velocity_x,
            velocity_y,
        }
    }

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

            let rgba = if inside_the_box {
                [0x5e, 0x48, 0xe8, 0xff]
            } else {
                [0x48, 0xb2, 0xe8, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}

struct State {
    update_called: u8,
    render_called: u8,
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

    let moving_box = Rc::new(RefCell::new(MovingBox::new(
        0,
        0,
        50,
        1,
        1,
    )));

    pixel_loop_tao(
        120,
        |moving_box, state| {
            let mut borrow_box = moving_box.borrow_mut();
            borrow_box.update();
            state.update_called += 1;

            // std::thread::sleep(Duration::from_millis(4));
            // println!("Update");
        },
        |pixels, moving_box, state| {
            let borrow_box = moving_box.borrow();
            borrow_box.render(pixels.frame_mut());
            state.render_called += 1;
            let _ = pixels.render();
            // std::thread::sleep(Duration::from_millis(16));
            // println!("Render");
        },
        event_loop,
        pixels,
        moving_box,
        state,
    );
}

fn pixel_loop_tao(
    update_fps: u64, 
    update: fn(Rc<RefCell<MovingBox>>, &mut State), 
    render: fn(&mut Pixels, Rc<RefCell<MovingBox>>, &mut State),
    event_loop: EventLoop<()>,
    mut pixels: Pixels,
    moving_box: Rc<RefCell<MovingBox>>,
    mut state: State,
    
) {
    let mut t = Duration::default();
    let dt = Duration::from_nanos(1_000_000_000 / update_fps);

    let mut current_time = Instant::now();
    let mut accumulator = Duration::new(0, 0);

    event_loop.run(move |event, _, _| {
        match event {
            Event::MainEventsCleared => {
                let new_time = Instant::now();
                let mut frame_time = new_time - current_time;
                current_time = new_time;

                if frame_time > Duration::from_millis(100) {
                    frame_time = Duration::from_millis(100);
                }

                accumulator += frame_time;

                while accumulator >= dt {
                    update(Rc::clone(&moving_box), &mut state);
                    accumulator -= dt;
                    t += dt;
                }

                render(&mut pixels, Rc::clone(&moving_box), &mut state);

                if t > Duration::from_secs(1) {
                    println!("Update fps: {}", state.update_called);
                    println!("render fps: {}", state.render_called);
                    t = Duration::default();
                    state.update_called = 0;
                    state.render_called = 0;
                }
            }
            _ => {}
        }
    });
}
