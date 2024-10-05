pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r,
            g,
            b,
            a,
        }
    }
    pub fn as_list(&self) -> [u8; 4]{
        [self.r, self.g, self.b, self.a]
    }
}

