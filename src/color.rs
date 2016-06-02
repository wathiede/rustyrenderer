use draw;
use rand::random;

pub static WHITE: draw::RGB = draw::RGB {
    r: 255,
    g: 255,
    b: 255,
};
pub static RED: draw::RGB = draw::RGB {
    r: 255,
    g: 0,
    b: 0,
};
pub static GREEN: draw::RGB = draw::RGB {
    r: 0,
    g: 255,
    b: 0,
};
pub static BLUE: draw::RGB = draw::RGB {
    r: 0,
    g: 0,
    b: 255,
};

pub fn rand() -> draw::RGB {
    draw::RGB {
        r: random::<u8>(),
        g: random::<u8>(),
        b: random::<u8>(),
    }
}
