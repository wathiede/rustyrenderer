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

    let obj = wavefront::Object::read("obj/african_head.obj").unwrap();
    info!("Loading model {}", obj);

    let (width, height) = (800, 800);
    let mut im = draw::Image::new(width, height);
    let mut z_buffer = draw::DepthBuffer::new(width, height);
    {
        let mut shdr = shader::FlatShader::new(&obj, &mut im, &mut z_buffer, LIGHT_DIR.normalize());
        for f in &obj {
            shader::Shader::draw_face(&mut shdr, &f);
        }
    }

    im.flip_y();
    let out_path = Path::new("output.png");
    println!("Saving {}", out_path.display());
    imagefmt::write(out_path, im.w, im.h, ColFmt::RGB, &im.buf, ColType::Auto).unwrap();
}
