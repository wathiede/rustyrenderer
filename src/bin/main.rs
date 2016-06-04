#[macro_use]
extern crate log;
extern crate env_logger;
extern crate imagefmt;
extern crate rustyrenderer;

use imagefmt::{ColFmt, ColType};
use rustyrenderer::*;
use std::path::Path;

// line is useful for debugging
#[allow(dead_code)]
fn line(im: &mut draw::Image, v0: &math::Vec3f, v1: &math::Vec3f, c: draw::RGB) {
    let (w2, h2) = (im.w as f32 / 2., im.h as f32 / 2.);
    im.line(&math::Vec2i {
                x: ((v0.x + 1.) * w2) as i32,
                y: ((v0.y + 1.) * h2) as i32,
            },
            &math::Vec2i {
                x: ((v1.x + 1.) * w2) as i32,
                y: ((v1.y + 1.) * h2) as i32,
            },
            c);
}

static LIGHT_DIR: math::Vec3f = math::Vec3f {
    x: 0.,
    y: 0.,
    z: -1.,
};

fn main() {
    env_logger::init().unwrap();

    let obj = wavefront::Object::new("obj/african_head.obj").unwrap();
    info!("Loading model {}", obj);

    let model2screen = |im: &draw::Image, v: &math::Vec3f| {
        let (w2, h2) = (im.w as f32 / 2., im.h as f32 / 2.);
        // .trunc() necessary to prevent cracks.
        math::Vec3f {
            x: ((v.x + 1.) * w2).trunc(),
            y: ((v.y + 1.) * h2).trunc(),
            z: v.z,
        }
    };

    let (width, height) = (800, 800);
    let ref mut im = draw::Image::new(width, height);
    let ref mut z_buffer = draw::DepthBuffer::new(width, height);
    for f in obj {
        // XXX Ugly, clean this up with shaders.
        let ref v0 = f.vertices[0];
        let ref v1 = f.vertices[1];
        let ref v2 = f.vertices[2];
        let world_tri = [v0, v1, v2];
        let screen_tri = [model2screen(im, v0), model2screen(im, v1), model2screen(im, v2)];
        let n = math::cross(world_tri[2] - world_tri[0], world_tri[1] - world_tri[0]);
        let alpha = math::dot(n.normalize(), LIGHT_DIR.normalize());
        if alpha < 0. {
            continue;
        }
        let c = draw::RGB {
            r: (alpha * 255.) as u8,
            g: (alpha * 255.) as u8,
            b: (alpha * 255.) as u8,
        };
        im.triangle(&screen_tri, c, z_buffer);
    }

    im.flip_y();
    let out_path = Path::new("output.png");
    println!("Saving {}", out_path.display());
    imagefmt::write(out_path, im.w, im.h, ColFmt::RGB, &im.buf, ColType::Auto).unwrap();
}
