use std::cmp;
use std::fmt;
use std::ops;

#[derive(Clone,Debug)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

#[derive(Copy,Clone,Debug)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// XXX Only for use during development, should go away soon.
impl Vec3f {
    pub fn to_vec2i(&self) -> Vec2i {
        Vec2i {
            x: self.x as i32,
            y: self.y as i32,
        }
    }

    pub fn zero() -> Vec3f {
        Vec3f {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }
}

impl fmt::Display for Vec3f {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "<{} {} {}>", self.x, self.y, self.z)
    }
}

impl ops::Index<usize> for Vec3f {
    type Output = f32;

    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Attempt to index outside of range 0..3: {}", idx),
        }
    }
}

impl ops::IndexMut<usize> for Vec3f {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match idx {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Attempt to index outside of range 0..3: {}", idx),
        }
    }
}

impl ops::Sub for Vec3f {
    type Output = Vec3f;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3f {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Add for Vec3f {
    type Output = Vec3f;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3f {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl cmp::PartialEq for Vec3f {
    fn eq(&self, rhs: &Self) -> bool {
        self.x == rhs.x && self.y == rhs.y && self.z == rhs.z
    }
}


impl Vec3f {
    pub fn normalize(&self) -> Vec3f {
        self.scale(1. / self.length())
    }
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn scale(&self, a: f32) -> Vec3f {
        Vec3f {
            x: self.x * a,
            y: self.y * a,
            z: self.z * a,
        }
    }
}

pub fn dot(v0: Vec3f, v1: Vec3f) -> f32 {
    v0.x * v1.x + v0.y * v1.y + v0.z * v1.z
}

pub fn cross(v0: Vec3f, v1: Vec3f) -> Vec3f {
    Vec3f {
        x: v0.y * v1.z - v0.z * v1.y,
        y: v0.z * v1.x - v0.x * v1.z,
        z: v0.x * v1.y - v0.y * v1.x,
    }
}

pub fn barycentric(tri: &[Vec3f; 3], p: Vec3f) -> Vec3f {
    let (a, b, c) = (tri[0], tri[1], tri[2]);
    let u = cross(Vec3f {
                      x: c.x - a.x,
                      y: b.x - a.x,
                      z: a.x - p.x,
                  },
                  Vec3f {
                      x: c.y - a.y,
                      y: b.y - a.y,
                      z: a.y - p.y,
                  });
    // info!("bc\np {:?}\ntri {:?}\nu {}", p, tri, u);
    if u.z.abs() > 0. {
        return Vec3f {
            x: 1. - (u.x + u.y) / u.z,
            y: u.y / u.z,
            z: u.x / u.z,
        };
    }
    // triangle is degenerate, in this case return something with negative coordinates
    Vec3f {
        x: -1.,
        y: 1.,
        z: 1.,
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_barycentric() {
        let p = Vec3f {
            x: 720.,
            y: 720.,
            z: 0.,
        };
        let tri = &[Vec3f {
                        x: 700.,
                        y: 700.,
                        z: 99268.22,
                    },
                    Vec3f {
                        x: 800.,
                        y: 700.,
                        z: 100763.49,
                    },
                    Vec3f {
                        x: 700.,
                        y: 800.,
                        z: 101040.8,
                    }];
        let v = barycentric(tri, p);
        assert_eq!(v,
                   Vec3f {
                       x: 0.6,
                       y: 0.2,
                       z: 0.2,
                   });
    }
}

// Matrix is a 4x4 matrix type.  It accessed using the index trait, i.e. `m[(r, c)]`.
#[derive(Clone,Debug)]
pub struct Matrix {
    v: [[f32; 4]; 4],
}

impl Matrix {
    pub fn new() -> Matrix {
        Matrix {
            v: [[0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0]],
        }
    }

    pub fn identity() -> Matrix {
        Matrix {
            v: [[1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]],
        }
    }

    // TODO(wathiede): make this an implementation of ops::Mul somehow.
    pub fn transform(&self, rhs: Vec3f) -> Vec3f {
        let inp = [rhs.x, rhs.y, rhs.z, 1.];
        let mut out = [0.; 4];
        for i in 0..4 {
            for k in 0..4 {
                out[i] += self[(i, k)] * inp[k];
            }
        }

        for i in 0..3 {
            out[i] *= 1. / out[3];
        }

        Vec3f {
            x: out[0],
            y: out[1],
            z: out[2],
        }
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(write!(f, "<\n"));
        for r in 0..4 {
            try!(write!(f, "["));
            try!(write!(f, "{:7.2}", self[(r, 0)]));
            for c in 1..4 {
                try!(write!(f, " {:+7.2}", self[(r, c)]));
            }
            try!(write!(f, "]\n"));
        }
        write!(f, ">")
    }
}

impl ops::Index<(usize, usize)> for Matrix {
    type Output = f32;

    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        let (r, c) = idx;
        &self.v[r][c]
    }
}

impl ops::IndexMut<(usize, usize)> for Matrix {
    fn index_mut<'a>(&'a mut self, idx: (usize, usize)) -> &'a mut Self::Output {
        let (r, c) = idx;
        &mut self.v[r][c]
    }
}

impl ops::Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut out = Matrix::new();
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    out[(i, j)] += self[(i, k)] * rhs[(k, j)];
                }
            }
        }
        out
    }
}
