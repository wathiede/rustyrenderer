extern crate rustyrenderer;

fn main() {
    let im = rustyrenderer::draw::RGBImage {
        w: 256,
        h: 256,
        buf: vec![rustyrenderer::draw::RGB{r:255,g:0,b:0}; 256*256],
    };
    println!("{:?}", im);
}
