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
    let red = draw::RGB {
        r: 255,
        g: 0,
        b: 0,
    };
    let green = draw::RGB {
        r: 0,
        g: 255,
        b: 0,
    };
    let blue = draw::RGB {
        r: 0,
        g: 0,
        b: 255,
    };

    let (width, height) = (128, 128);
    let mut im = draw::Image::new(width, height);
    im.line(draw::Vec2i { x: 80, y: 40 },
            draw::Vec2i { x: 13, y: 20 },
            green);
    im.line(draw::Vec2i { x: 13, y: 20 },
            draw::Vec2i { x: 80, y: 40 },
            white);
    im.line(draw::Vec2i { x: 40, y: 80 },
            draw::Vec2i { x: 20, y: 13 },
            blue);
    im.line(draw::Vec2i { x: 20, y: 13 },
            draw::Vec2i { x: 40, y: 80 },
            red);

    im.flip_y();
    let out_path = Path::new("output.png");
    println!("Saving {}", out_path.display());
    imagefmt::write(out_path, im.w, im.h, ColFmt::RGB, &im.buf, ColType::Auto).unwrap();
}
