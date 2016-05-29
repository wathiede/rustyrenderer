use std::fmt;

#[derive(Copy,Clone)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl fmt::Debug for RGB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}


#[derive(Debug)]
pub struct Image {
    pub w: usize,
    pub h: usize,
    pub buf: Vec<u8>,
}

impl Image {
    pub fn new(w: usize, h: usize) -> Self {
        return Image {
            w: w,
            h: h,
            buf: vec![0; w*h*3],
        };
    }
    pub fn set(&mut self, x: usize, y: usize, c: RGB) {
        let off = (x + y * self.w) * 3;
        self.buf[off + 0] = c.r;
        self.buf[off + 1] = c.g;
        self.buf[off + 2] = c.b;
    }
    pub fn line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, c: RGB) {
        for i in 0..100 {
            let t = i as f64 / 100.;
            let x = x0 as f64 * (1. - t) + x1 as f64 * t;
            let y = y0 as f64 * (1. - t) + y1 as f64 * t;
            self.set(x as usize, y as usize, c);
        }
    }
    pub fn flip_y(&mut self) {
        for y in 0..self.h / 2 {
            for x in 0..self.w {
                let up = (x + y * self.w) * 3;
                let bot = (x + (self.h - y - 1) * self.w) * 3;
                self.buf.swap(up + 0, bot + 0);
                self.buf.swap(up + 1, bot + 1);
                self.buf.swap(up + 2, bot + 2);
            }
        }
    }
}
