extern crate imagefmt;
extern crate rustyrenderer;

use imagefmt::{ColFmt, ColType};
use rustyrenderer::draw;
use std::path::Path;

fn main() {
    let (width, height) = (512, 512);
    let mut im = draw::Image::new(width, height);
    for x in 0..width {
        for y in 0..height {
            im.set(x,
                   y,
                   draw::RGB {
                       r: x as u8,
                       g: y as u8,
                       b: (x ^ y) as u8,
                   });
        }
    }
    let out_path = Path::new("output.png");
    println!("Saving {}", out_path.display());
    imagefmt::write(out_path, im.w, im.h, ColFmt::RGB, &im.buf, ColType::Auto).unwrap();
}
