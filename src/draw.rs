use math;
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
        if x >= self.w || y >= self.h {
            error!("Out of bounds set pixel {},{} size {}x{}",
                   x,
                   y,
                   self.w,
                   self.h);
            return;
        }
        let off = (x + y * self.w) * 3;
        self.buf[off + 0] = c.r;
        self.buf[off + 1] = c.g;
        self.buf[off + 2] = c.b;
    }

    pub fn line(&mut self, p0: &math::Vec2i, p1: &math::Vec2i, c: RGB) {
        debug!("p0 {:?} p1 {:?}", p0, p1);
        // Taller than wide line.
        let steep = (p0.x - p1.x).abs() < (p0.y - p1.y).abs();
        let (x0, y0, x1, y1) = {
            let &math::Vec2i { x: x0, y: y0 } = p0;
            let &math::Vec2i { x: x1, y: y1 } = p1;
            if steep {
                // Taller than wide, swap x & y.
                if y0 > y1 {
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
        for x in x0..x1 {
            let t = (x - x0) as f32 / (x1 - x0) as f32;
            let y = y0 as f32 * (1. - t) + y1 as f32 * t;
            let (xs, ys) = if steep {
                (y as usize, x as usize)
            } else {
                (x as usize, y as usize)
            };
            self.set(xs, ys, c);
        }

    }

    pub fn triangle(&mut self, tri: &[math::Vec3f; 3], c: RGB) {
        let ref v0 = tri[0].to_vec2i();
        let ref v1 = tri[1].to_vec2i();
        let ref v2 = tri[2].to_vec2i();
        use std::cmp::{max, min};
        let (x_min, x_max) = (min(min(v0.x, v1.x), v2.x), max(max(v0.x, v1.x), v2.x));
        let (y_min, y_max) = (min(min(v0.y, v1.y), v2.y), max(max(v0.y, v1.y), v2.y));
        for y in y_min..y_max {
            for x in x_min..x_max {
                let bc = math::barycentric(tri, math::Vec2i { x: x, y: y });
                if bc.x < 0. || bc.y < 0. || bc.z < 0. {
                    continue;
                }
                self.set(x as usize, y as usize, c);
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
