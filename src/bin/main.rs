#[macro_use]
extern crate log;
extern crate env_logger;
extern crate imagefmt;
extern crate rustyrenderer;

use imagefmt::{ColFmt, ColType};
use rustyrenderer::*;
use std::path::Path;

static LIGHT_DIR: math::Vec3f = math::Vec3f {
    x: 1.,
    y: 1.,
    z: 1.,
};

static EYE_DIR: math::Vec3f = math::Vec3f {
    x: 1.,
    y: 1.,
    z: 3.,
};

static CENTER_DIR: math::Vec3f = math::Vec3f {
    x: 0.,
    y: 0.,
    z: 0.,
};

static UP_DIR: math::Vec3f = math::Vec3f {
    x: 0.,
    y: 1.,
    z: 0.,
};

fn main() {
    env_logger::init().unwrap();

    // let model = "obj/pal.obj";
    let model = "obj/african_head.obj";
    let obj = wavefront::Object::read(model).unwrap();
    // let obj = wavefront::Object::read().unwrap();
    info!("Loading model {}", obj);

    let (width, height) = (1024, 1024);
    let mut im = draw::Image::new(width, height);
    let mut z_buffer = draw::DepthBuffer::new(width, height);
    let mut world = shader::World::new();
    world.set_viewport(width / 8, height / 8, 3 * width / 4, 3 * height / 4);
    world.set_light_dir(LIGHT_DIR);
    world.look_at(EYE_DIR, CENTER_DIR, UP_DIR);
    info!("viewport  : {}", world.viewport);
    info!("projection: {}", world.projection);
    info!("model_view: {}", world.model_view);
    {
        let mut shdr = shader::FlatShader::new(&obj, &mut im, &mut z_buffer);
        for f in &obj {
            shader::Shader::draw_face(&mut shdr, &world, &f);
        }
    }

    im.flip_y();
    let out_path = Path::new("output.png");
    println!("Saving {}", out_path.display());
    imagefmt::write(out_path, im.w, im.h, ColFmt::RGB, &im.buf, ColType::Auto).unwrap();
}
