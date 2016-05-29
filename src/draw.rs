use std::fmt;

// TODO(wathiede): understand why this Clone is necessary to vec!
#[derive(Copy,Clone)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl fmt::Debug for RGB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}


#[derive(Debug)]
pub struct Image {
    pub w: usize,
    pub h: usize,
    pub buf: Vec<u8>,
}

impl Image {
    pub fn new(w: usize, h: usize) -> Self {
        return Image {
            w: w,
            h: h,
            buf: vec![0; w*h*3],
        };
    }
    pub fn set(&mut self, x: usize, y: usize, p: RGB) {
        let off = (x + y * self.h) * 3;
        self.buf[off + 0] = p.r;
        self.buf[off + 1] = p.g;
        self.buf[off + 2] = p.b;
    }
    // fn buffer_size(&self) -> usize {
    // self.w * self.h * std::mem::size_of::<Pixel>()
    // }
    //

    // pub fn get_bytes(&self) -> Vec<u8> {
    // return self.buf.clone();
    // }
}
