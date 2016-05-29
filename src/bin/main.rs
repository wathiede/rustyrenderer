extern crate imagefmt;
extern crate rustyrenderer;

use imagefmt::{ColFmt, ColType};
use rustyrenderer::draw;
use std::path::Path;

fn main() {
    let white = draw::RGB {
        r: 255,
        g: 255,
        b: 255,
    };

    let (width, height) = (512, 512);
    let mut im = draw::Image::new(width, height);
    im.line(13, 20, 80, 40, white);

    im.flip_y();
    let out_path = Path::new("output.png");
    println!("Saving {}", out_path.display());
    imagefmt::write(out_path, im.w, im.h, ColFmt::RGB, &im.buf, ColType::Auto).unwrap();
}
