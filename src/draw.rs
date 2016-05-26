use std::fmt;

// TODO(wathiede): understand why this Clone is necessary to vec!
#[derive(Clone)]
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


pub type RGBImage = Image<RGB>;

#[derive(Debug)]
pub struct Image<Pixel> {
    pub w: usize,
    pub h: usize,
    pub buf: Vec<Pixel>,
}
