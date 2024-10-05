use std::time::{Instant, Duration};
use crate::GameObject;
use pixels::Pixels;

struct State {
    update_called: u8,
    render_called: u8,
}


pub struct PixelLoop {
    objects: Vec<Box<dyn GameObject>>,
    state: State,
    accumulator: Duration,
    current_time: Instant,
    time: Duration,
    dt: Duration,
    pixels: Pixels,
}

impl PixelLoop {
    pub fn new(update_fps: u64, pixels: Pixels) -> Self {
        let state = State { 
            update_called: 0,
            render_called: 0,
        };
        Self {
            pixels,
            state,
            accumulator: Duration::new(0, 0),
            current_time: Instant::now(),
            time: Duration::default(),
            dt: Duration::from_nanos(1_000_000_000 / update_fps),
            objects: Vec::new(),
        }
    }

    pub fn next_loop(&mut self) {
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
    
    pub fn add_object(&mut self, object: Box<dyn GameObject>) {
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
