use std::iter;
use std::error;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::num;

use math::Vec2i;
use math::Vec3f;

type Vertex = Vec3f;

#[derive(Clone)]
struct FaceIndex {
    // TODO(wathiede): unpublish these when shader implemented.
    pub vertexIndices: Vec<usize>,
    pub texCoordIndices: Vec<usize>,
    pub normalIndices: Vec<usize>,
}

impl FaceIndex {
    fn new() -> Self {
        FaceIndex {
            vertexIndices: Vec::new(),
            texCoordIndices: Vec::new(),
            normalIndices: Vec::new(),
        }
    }
}

impl fmt::Display for FaceIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        writeln!(f,
                 "{} vert idx {} tex idx {} norm idx",
                 self.vertexIndices.len(),
                 self.texCoordIndices.len(),
                 self.normalIndices.len())
    }
}

// TODO(wathiede): rename 'Triangle'?
pub struct Face {
    pub vertices: [Vec3f; 3],
}

impl fmt::Display for Face {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "{:?} vertices", self.vertices)
    }
}

pub struct Object {
    vertices: Vec<Vertex>,
    faces: Vec<FaceIndex>,
}

#[derive(Debug)]
enum ErrorRepr {
    ParseError(String),
    ParseFloatError(num::ParseFloatError),
    ParseIntError(num::ParseIntError),
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

impl From<num::ParseIntError> for ObjectError {
    fn from(err: num::ParseIntError) -> ObjectError {
        ObjectError {
            desc: "Format error",
            cause: ErrorRepr::ParseIntError(err),
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
            ErrorRepr::ParseIntError(ref err) => Some(err as &error::Error),
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

        let mut obj = Object {
            vertices: Vec::new(),
            faces: Vec::new(),
        };

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

    pub fn vertex(&self, idx: usize) -> Vec3f {
        self.vertices[idx].clone()
    }

    fn parse_line(&mut self, l: String) -> Result<(), ObjectError> {
        let p: Vec<_> = l.split_whitespace().collect();
        if p.is_empty() {
            return Ok(());
        }
        match p[0] {
            "#" => info!("Comment {:?}", p),
            "f" => return self.add_face(p),
            "v" => return self.add_vertex(p),
            "vn" => debug!("Vertex normal {:?}", p),
            "vt" => debug!("Tex {:?}", p),
            _ => info!("Unknown line type: {:?}", p),
        }
        Ok(())
    }

    fn add_face(&mut self, p: Vec<&str>) -> Result<(), ObjectError> {
        debug!("Face {:?}", p);
        // TODO(wathiede): add support for quad faces, triangles only for now.
        if p.len() != 4 {
            return Err(ObjectError {
                desc: "Bad line", /* desc: format!("Got {} vert components, expected 4: {:?}", p.len(), p).to_string(), */
                cause: ErrorRepr::ParseError(p.join(" ")),
            });
        };
        let mut f = FaceIndex::new();
        for n in p.iter().skip(1) {
            for (i, c) in n.split("/").enumerate() {
                // Face indices in wavefront object files are 1-based.
                let idx = try!(c.parse::<usize>()) - 1;
                match (i, idx) {
                    (0, idx) => f.vertexIndices.push(idx),
                    (1, idx) => f.texCoordIndices.push(idx),
                    (2, idx) => f.normalIndices.push(idx),
                    (_, _) => panic!("Found more than 3 components in face"),
                }
            }
        }
        self.faces.push(f);
        Ok(())
    }

    fn add_vertex(&mut self, p: Vec<&str>) -> Result<(), ObjectError> {
        debug!("Vertex {:?}", p);
        // "v <x> <y> <z>"
        if p.len() != 4 {
            return Err(ObjectError {
                desc: "Bad line", /* desc: format!("Got {} vert components, expected 4: {:?}", p.len(), p).to_string(), */
                cause: ErrorRepr::ParseError(p.join(" ")),
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
        writeln!(f,
                 "{} vertices {} faces",
                 self.vertices.len(),
                 self.faces.len())
    }
}

pub struct ObjectIter {
    obj: Object,
    idx: usize,
}

impl iter::Iterator for ObjectIter {
    type Item = Face;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.obj.faces.len() {
            return None;
        }
        let ref f_idx = self.obj.faces[self.idx];
        // TODO(wathiede): add texcoord/normal values.
        let face = Face {
            vertices: [self.obj.vertex(f_idx.vertexIndices[0]),
                       self.obj.vertex(f_idx.vertexIndices[1]),
                       self.obj.vertex(f_idx.vertexIndices[2])],
        };
        self.idx += 1;
        Some(face)
    }
}

impl iter::IntoIterator for Object {
    type Item = Face;
    type IntoIter = ObjectIter;

    fn into_iter(self) -> Self::IntoIter {
        info!("Creating into_iter");
        ObjectIter {
            obj: self,
            idx: 0,
        }
    }
}
