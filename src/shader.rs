use color;
use draw;
use math;
use wavefront;

static DEPTH_RESOLUTION: f32 = 65_536.;

pub struct World {
    light_dir: math::Vec3f,
    pub model_view: math::Matrix,
    pub viewport: math::Matrix,
    pub projection: math::Matrix,
}

impl World {
    pub fn new() -> Self {
        World {
            light_dir: math::Vec3f {
                x: 0.,
                y: 0.,
                z: -1.,
            },
            model_view: math::Matrix::identity(),
            viewport: math::Matrix::identity(),
            projection: math::Matrix::identity(),
        }
    }

    pub fn set_light_dir(&mut self, light_dir: math::Vec3f) {
        self.light_dir = light_dir;
    }

    pub fn look_at(&mut self, eye: math::Vec3f, center: math::Vec3f, up: math::Vec3f) {
        let z = (eye - center).normalize();
        let x = math::cross(up, z).normalize();
        let y = math::cross(z, x).normalize();
        let mut m_inv = math::Matrix::identity();
        let mut t_r = math::Matrix::identity();
        for i in 0..3 {
            m_inv[(0, i)] = x[i];
            m_inv[(1, i)] = y[i];
            m_inv[(2, i)] = z[i];
            t_r[(i, 3)] = -center[i];
        }
        self.model_view = m_inv * t_r;
        self.projection[(3, 2)] = -1. / (eye - center).length();
    }

    pub fn set_viewport(&mut self, x_off: usize, y_off: usize, width: usize, height: usize) {
        let x = x_off as f32;
        let y = y_off as f32;
        let w = width as f32;
        let h = height as f32;
        let mut m = math::Matrix::identity();
        m[(0, 3)] = x + w / 2.;
        m[(1, 3)] = y + h / 2.;
        m[(2, 3)] = DEPTH_RESOLUTION / 2.;

        m[(0, 0)] = w / 2.;
        m[(1, 1)] = h / 2.;
        m[(2, 2)] = DEPTH_RESOLUTION / 2.;
        self.viewport = m;
    }
}

pub trait Shader {
    // vertex sets per-face shader state in preparation for fragment evaluation.
    fn vertex(&mut self, world: &World, f: &wavefront::Face);
    // fragment evaluates the color of a pixel fragment. It returns None if the pixel should be
    // discarded, i.e. culled as a back facing polygon.
    fn fragment(&self, world: &World, bc: math::Vec3f) -> Option<draw::RGB>;
    // draw_face calls vertex on the face, and then fragment per-pixel.
    fn draw_face(&mut self, world: &World, f: &wavefront::Face);
}

pub struct FlatShader<'a> {
    // Uniform values.
    obj: &'a wavefront::Object,
    im: &'a mut draw::Image,
    z_buffer: &'a mut draw::DepthBuffer,

    // Varying values, written by vertex shader, read by fragment shader
    // Per-face lighting scalar.
    intensity: f32,
    // Texture UV at fragment.
    uvs: [math::Vec3f; 3],
    // Normal UV at fragment.
    ns: [math::Vec3f; 3],
    // screen space vertices of the face.
    screen_verts: [math::Vec3f; 3],
}

impl<'a> FlatShader<'a> {
    pub fn new(obj: &'a wavefront::Object,
               im: &'a mut draw::Image,
               z_buffer: &'a mut draw::DepthBuffer)
               -> Self {
        FlatShader {
            obj: obj,
            im: im,
            z_buffer: z_buffer,
            intensity: 1.,
            uvs: [math::Vec3f::zero(), math::Vec3f::zero(), math::Vec3f::zero()],
            ns: [math::Vec3f::zero(), math::Vec3f::zero(), math::Vec3f::zero()],
            screen_verts: [math::Vec3f::zero(), math::Vec3f::zero(), math::Vec3f::zero()],
        }
    }
}

impl<'a> Shader for FlatShader<'a> {
    fn vertex(&mut self, world: &World, f: &wavefront::Face) {

        let mut world_tri = [math::Vec3f::zero(), math::Vec3f::zero(), math::Vec3f::zero()];
        // TODO(wathiede): move this to compute once.
        let m = world.viewport.clone() * world.projection.clone() * world.model_view.clone();
        let mut intensity = 0.;
        for i in 0..3 {
            world_tri[i] = f.vertices[i];
            debug!("m {}", m);
            self.screen_verts[i] = m.transform(f.vertices[i]);
            debug!("v {} -> {}", f.vertices[i], self.screen_verts[i]);
            // self.screen_verts[i] = self.model2screen(f.vertices[i]);
            self.uvs[i] = f.texcoords[i];
            let n = f.normals[i];
            intensity += math::dot(n, world.light_dir.normalize());
            self.ns[i] = n;
        }
        self.intensity = intensity / 3.;
    }

    fn fragment(&self, _world: &World, bc: math::Vec3f) -> Option<draw::RGB> {
        if self.intensity < 0. {
            return None;
        }
        let uv = self.uvs[0].scale(bc.x) + self.uvs[1].scale(bc.y) + self.uvs[2].scale(bc.z);
        let c = self.obj.sample(uv);
        // TODO(wathiede): perform texture lookup and set color appropriately.

        Some(draw::RGB {
            r: (c.r as f32 * self.intensity) as u8,
            g: (c.g as f32 * self.intensity) as u8,
            b: (c.b as f32 * self.intensity) as u8,
        })
    }

    fn draw_face(&mut self, world: &World, f: &wavefront::Face) {
        self.vertex(world, f);

        let ref tri = self.screen_verts;
        let ref v0 = tri[0].to_vec2i();
        let ref v1 = tri[1].to_vec2i();
        let ref v2 = tri[2].to_vec2i();

        if false {
            self.im.line(v0, v1, color::RED);
            self.im.line(v0, v2, color::GREEN);
            self.im.line(v1, v2, color::BLUE);
        }

        use std::cmp::{max, min};
        let (x_min, x_max) = (min(min(v0.x, v1.x), v2.x), max(max(v0.x, v1.x), v2.x));
        let (y_min, y_max) = (min(min(v0.y, v1.y), v2.y), max(max(v0.y, v1.y), v2.y));
        debug!("Tri BBox x {},{} y {},{}", x_min, x_max, y_min, y_max);

        let mut debug_drawn = 0;
        let mut debug_discard = 0;
        let mut debug_outside = 0;
        for y in y_min..y_max + 1 {
            for x in x_min..x_max + 1 {
                let bc = math::barycentric(tri,
                                           math::Vec3f {
                                               x: x as f32,
                                               y: y as f32,
                                               z: 0.,
                                           });
                if bc.x < 0. || bc.y < 0. || bc.z < 0. {
                    // Outside the triangle.
                    debug_outside += 1;
                    continue;
                }

                let (sx, sy) = (x as usize, y as usize);
                let z = tri[0].z * bc.x + tri[1].z * bc.y + tri[2].z * bc.z;
                // Z test passes, draw pixel
                if self.z_buffer.get(sx, sy) < z {
                    match self.fragment(world, bc) {
                        Some(c) => {
                            self.z_buffer.set(sx, sy, z);
                            self.im.set(sx, sy, c);
                            debug_drawn += 1;
                        }
                        // Fragment says to discard, don't update z-buffer.
                        None => debug_discard += 1,
                    }
                }
            }
        }
        debug!("DEBUG discarded {} outside {} drawn {}",
              debug_discard,
              debug_outside,
              debug_drawn,
              );
        // panic!();
    }
}
