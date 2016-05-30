use std::error;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::num;

use math::Vec3f;

type Vertex = Vec3f;

pub struct Object {
    vertices: Vec<Vertex>,
}

#[derive(Debug)]
enum ErrorRepr {
    ParseError(&'static str),
    ParseFloatError(num::ParseFloatError),
    IoError(io::Error),
}

#[derive(Debug)]
pub struct ObjectError {
    desc: &'static str,
    cause: ErrorRepr,
}

impl From<io::Error> for ObjectError {
    fn from(err: io::Error) -> ObjectError {
        ObjectError {
            desc: "IO error",
            cause: ErrorRepr::IoError(err),
        }
    }
}

impl From<num::ParseFloatError> for ObjectError {
    fn from(err: num::ParseFloatError) -> ObjectError {
        ObjectError {
            desc: "Format error",
            cause: ErrorRepr::ParseFloatError(err),
        }
    }
}

impl error::Error for ObjectError {
    fn description(&self) -> &str {
        self.desc
    }

    fn cause(&self) -> Option<&error::Error> {
        match self.cause {
            ErrorRepr::ParseError(_) => None,
            ErrorRepr::ParseFloatError(ref err) => Some(err as &error::Error),
            ErrorRepr::IoError(ref err) => Some(err as &error::Error),
        }
    }
}

impl fmt::Display for ObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.desc.fmt(f)
    }
}

impl Object {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, ObjectError> {
        let f = try!(File::open(path));
        let f = BufReader::new(f);

        let mut obj = Object { vertices: Vec::new() };
        for line in f.lines() {
            match line {
                Ok(l) => {
                    try!(obj.parse_line(l));
                }
                Err(e) => {
                    return Err(ObjectError {
                        desc: "failed to read line",
                        cause: ErrorRepr::IoError(e),
                    })
                }
            }
        }

        Ok(obj)
    }

    fn parse_line(&mut self, l: String) -> Result<(), ObjectError> {
        let p: Vec<_> = l.split_whitespace().collect();
        if p.is_empty() {
            return Ok(());
        }
        match p[0] {
            "#" => info!("Comment {:?}", p),
            "f" => info!("Face {:?}", p),
            "v" => return self.add_vertex(p),
            "vt" => info!("Tex {:?}", p),
            _ => info!("Unknown line type: {:?}", p),
        }
        Ok(())
    }

    fn add_vertex(&mut self, p: Vec<&str>) -> Result<(), ObjectError> {
        info!("Vertex {:?}", p);
        if p.len() != 4 {
            return Err(ObjectError {
                desc: "Bad line", /* desc: format!("Got {} vert components, expected 4: {:?}", p.len(), p).to_string(), */
                cause: ErrorRepr::ParseError("TODO SOMETHING GOES HERE"),
            });
        };
        let x = try!(p[1].parse::<f32>());
        let y = try!(p[2].parse::<f32>());
        let z = try!(p[3].parse::<f32>());
        self.vertices.push(Vertex { x: x, y: y, z: z });
        Ok(())
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "{} vertices", self.vertices.len())
    }
}
