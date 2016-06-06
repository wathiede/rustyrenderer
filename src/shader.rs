use draw;
use math;
use wavefront;

pub trait Shader {
    // vertex sets per-face shader state in preparation for fragment evaluation.
    // TODO(wathiede): rename face?
    fn vertex(&mut self, f: &wavefront::Face);
    // fragment evaluates the color of a pixel fragment. It returns None if the pixel should be
    // discarded, i.e. culled as a back facing polygon.
    fn fragment(&self, bc: math::Vec3f) -> Option<draw::RGB>;
    // draw_face calls vertex on the face, and then fragment per-pixel.
    fn draw_face(&mut self, f: &wavefront::Face);
}

pub struct FlatShader<'a> {
    // Uniform values.
    obj: &'a wavefront::Object,
    im: &'a mut draw::Image,
    z_buffer: &'a mut draw::DepthBuffer,
    light_dir: math::Vec3f,

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
               z_buffer: &'a mut draw::DepthBuffer,
               light_dir: math::Vec3f)
               -> Self {
        FlatShader {
            obj: obj,
            im: im,
            z_buffer: z_buffer,
            light_dir: light_dir,
            intensity: 1.,
            uvs: [math::Vec3f::zero(), math::Vec3f::zero(), math::Vec3f::zero()],
            ns: [math::Vec3f::zero(), math::Vec3f::zero(), math::Vec3f::zero()],
            screen_verts: [math::Vec3f::zero(), math::Vec3f::zero(), math::Vec3f::zero()],
        }
    }

    // TODO(wathiede): replace with proper perspective transform logic.
    fn model2screen(&self, v: math::Vec3f) -> math::Vec3f {
        let (w2, h2) = (self.im.w as f32 / 2., self.im.h as f32 / 2.);
        // .trunc() necessary to prevent cracks.
        math::Vec3f {
            x: ((v.x + 1.) * w2).trunc(),
            y: ((v.y + 1.) * h2).trunc(),
            z: v.z,
        }
    }
}

impl<'a> Shader for FlatShader<'a> {
    fn vertex(&mut self, f: &wavefront::Face) {

        let mut world_tri = [math::Vec3f::zero(), math::Vec3f::zero(), math::Vec3f::zero()];
        for i in 0..3 {
            world_tri[i] = f.vertices[i];
            self.screen_verts[i] = self.model2screen(f.vertices[i]);
            self.uvs[i] = f.texcoords[i];
        }

        // TODO(wathiede): replace this with per-vertex normals from model, falling back to
        // computing them as face normals.  That would be gouruad shading
        let n = math::cross(world_tri[2] - world_tri[0], world_tri[1] - world_tri[0]).normalize();
        for i in 0..3 {
            self.ns[i] = n;
        }
        self.intensity = math::dot(n.normalize(), self.light_dir.normalize());
    }

    fn fragment(&self, bc: math::Vec3f) -> Option<draw::RGB> {
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

    fn draw_face(&mut self, f: &wavefront::Face) {
        self.vertex(f);

        let ref tri = self.screen_verts;
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
                    // Outside the triangle.
                    continue;
                }

                let (sx, sy) = (x as usize, y as usize);
                let z = tri[0].z * bc.x + tri[1].z * bc.y + tri[2].z * bc.z;
                // Z test passes, draw pixel
                if self.z_buffer.get(sx, sy) < z {
                    match self.fragment(bc) {
                        Some(c) => {
                            self.z_buffer.set(sx, sy, z);
                            self.im.set(sx, sy, c);
                        }
                        // Fragment says to discard, don't update z-buffer.
                        None => {}
                    }
                }
            }
        }
    }
}
