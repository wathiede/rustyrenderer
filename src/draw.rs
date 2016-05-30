use std::fmt;
use math::Vec2i;

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
    pub fn line(&mut self, p0: Vec2i, p1: Vec2i, c: RGB) {
        // Taller than wide line.
        let steep = (p0.x - p1.x).abs() < (p0.y - p1.y).abs();
        let (x0, y0, x1, y1) = {
            let Vec2i { x: x0, y: y0 } = p0;
            let Vec2i { x: x1, y: y1 } = p1;
            if steep {
                if x0 > x1 {
                    // p0 to the right of p1, swap to we can render left to right.
                    (y1, x1, y0, x0)
                } else {
                    // p0 to the left of p1, return as-is.
                    (y0, x0, y1, x1)
                }
            } else {
                // Wider than tall line.
                if x0 > x1 {
                    // p0 to the right of p1, swap to we can render left to right.
                    (x1, y1, x0, y0)
                } else {
                    // p0 to the left of p1, return as-is.
                    (x0, y0, x1, y1)
                }
            }
        };
        let dx = (x1 - x0) as f64;
        let dy = (y1 - y0) as f64;
        let derror2 = dy.abs() * 2.;
        let mut error2 = 0.;
        let mut y = y0;
        for x in x0..x1 {
            let (xs, ys) = if steep {
                (y as usize, x as usize)
            } else {
                (x as usize, y as usize)
            };
            self.set(xs, ys, c);
            error2 += derror2;
            if error2 > dx {
                if y1 > y0 {
                    y += 1;
                } else {
                    y += -1;
                };
                error2 -= dx * 2.;
            }
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
