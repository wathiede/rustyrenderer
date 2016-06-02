
#[derive(Clone,Debug)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone,Debug)]
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
}

pub fn cross(v0: Vec3f, v1: Vec3f) -> Vec3f {
    Vec3f {
        x: v0.y * v1.z - v0.z * v1.y,
        y: v0.z * v1.x - v0.x * v1.z,
        z: v0.x * v1.y - v0.y * v1.x,
    }
}

pub fn barycentric(tri: &[Vec3f; 3], p: Vec2i) -> Vec3f {
    let u = cross(Vec3f {
                      x: tri[2].x - tri[0].x,
                      y: tri[1].x - tri[0].x,
                      z: tri[0].x - p.x as f32,
                  },
                  Vec3f {
                      x: tri[2].y - tri[0].y,
                      y: tri[1].y - tri[0].y,
                      z: tri[0].y - p.y as f32,
                  });
    if u.z.abs() < 1. {
        return Vec3f {
            x: -1.,
            y: 1.,
            z: 1.,
        }; // triangle is degenerate, in this case return smth with negative coordinates
    }
    Vec3f {
        x: 1. - (u.x + u.y) / u.z,
        y: u.y / u.z,
        z: u.x / u.z,
    }
}
