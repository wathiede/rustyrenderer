#[macro_use]
extern crate log;
extern crate env_logger;
extern crate imagefmt;
extern crate rustyrenderer;

use imagefmt::{ColFmt, ColType};
use rustyrenderer::color;
use rustyrenderer::draw;
use rustyrenderer::math;
use rustyrenderer::wavefront;
use std::path::Path;

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

fn main() {
    env_logger::init().unwrap();

    let obj = wavefront::Object::new("obj/african_head.obj").unwrap();
    info!("Loading model {}", obj);

    let model2screen = |im: &draw::Image, v: &math::Vec3f| {
        let (w2, h2) = (im.w as f32 / 2., im.h as f32 / 2.);
        math::Vec3f {
            x: ((v.x + 1.) * w2),
            y: ((v.y + 1.) * h2),
            z: 0.,
        }
    };

    let (width, height) = (800, 800);
    let ref mut im = draw::Image::new(width, height);
    for f in obj {
        // XXX Ugly, clean this up with shaders.
        let ref v0 = f.vertices[0];
        let ref v1 = f.vertices[1];
        let ref v2 = f.vertices[2];
        let tri = [model2screen(im, v0), model2screen(im, v1), model2screen(im, v2)];
        im.triangle(&tri, color::rand());
    }

    im.flip_y();
    let out_path = Path::new("output.png");
    println!("Saving {}", out_path.display());
    imagefmt::write(out_path, im.w, im.h, ColFmt::RGB, &im.buf, ColType::Auto).unwrap();
}
