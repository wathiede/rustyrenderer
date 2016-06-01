
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
    pub fn to_vec2i(self) -> Vec2i {
        Vec2i {
            x: self.x as i32,
            y: self.y as i32,
        }
    }
}
