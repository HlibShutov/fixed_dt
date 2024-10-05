use crate::Color;
use crate::GameObject;
use crate::{WIDTH, HEIGHT};

pub struct MovingBox {
    box_x: i16,
    box_y: i16,
    size: i16,
    velocity_x: i16,
    velocity_y: i16,
    color: Color,
}

impl MovingBox {
    pub fn new(
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
