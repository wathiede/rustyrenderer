use std::iter;
use std::error;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::num;

use draw;
use imagefmt;
use math::Vec3f;

type Vertex = Vec3f;

#[derive(Clone)]
struct FaceIndex {
    v_idxs: Vec<usize>,
    t_idxs: Vec<usize>,
    n_idxs: Vec<usize>,
}

impl FaceIndex {
    fn new() -> Self {
        FaceIndex {
            v_idxs: Vec::new(),
            t_idxs: Vec::new(),
            n_idxs: Vec::new(),
        }
    }
}

impl fmt::Display for FaceIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f,
               "{} vert idx {} tex idx {} norm idx",
               self.v_idxs.len(),
               self.t_idxs.len(),
               self.n_idxs.len())
    }
}

// TODO(wathiede): rename 'Triangle'?
pub struct Face {
    pub vertices: [Vec3f; 3],
    pub texcoords: [Vec3f; 3],
    pub normals: [Vec3f; 3],
}

impl fmt::Display for Face {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?} vertices", self.vertices)
    }
}

#[derive(Debug)]
enum ErrorRepr {
    ParseError(String),
    ParseFloatError(num::ParseFloatError),
    ParseIntError(num::ParseIntError),
    IoError(io::Error),
    ImagefmtError(imagefmt::Error),
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

impl From<imagefmt::Error> for ObjectError {
    fn from(err: imagefmt::Error) -> ObjectError {
        ObjectError {
            desc: "image decode error",
            cause: ErrorRepr::ImagefmtError(err),
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
            ErrorRepr::ImagefmtError(ref err) => Some(err as &error::Error),
        }
    }
}

impl fmt::Display for ObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.desc.fmt(f)
    }
}

pub struct Object {
    vertices: Vec<Vertex>,
    texcoords: Vec<Vertex>,
    normals: Vec<Vertex>,
    faces: Vec<FaceIndex>,

    // TODO(wathiede): make this more flexible for multiple diffuse textures, and to support normal
    // and speculator maps.
    tex: draw::Texture2D,
}

impl Object {
    pub fn read<P: AsRef<Path>>(path: P) -> Result<Self, ObjectError> {
        let p = path.as_ref();
        let mut pb = p.to_path_buf();
        pb.set_file_name(p.file_stem().unwrap().to_string_lossy().to_string() + "_diffuse");
        pb.set_extension("tga");
        // TODO(wathiede): failure to load texture should be okay.
        let t = try!(draw::Texture2D::read(pb.as_path()));
        let mut obj = Object {
            vertices: Vec::new(),
            texcoords: Vec::new(),
            normals: Vec::new(),
            faces: Vec::new(),
            tex: t,
        };

        let f = try!(File::open(p));
        let f = BufReader::new(f);
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

    pub fn texcoord(&self, idx: usize) -> Vec3f {
        self.texcoords[idx].clone()
    }

    pub fn normal(&self, idx: usize) -> Vec3f {
        self.normals[idx].clone()
    }

    // Samples the currently active texture map at uv. Performs nearest neighbor sampling.
    pub fn sample(&self, uv: Vec3f) -> draw::RGB {
        self.tex.sample(uv)
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
            "vn" => return self.add_normal(p),
            "vt" => return self.add_texcoord(p),
            _ => info!("Unknown line type: {:?}", p),
        }
        Ok(())
    }

    fn add_face(&mut self, p: Vec<&str>) -> Result<(), ObjectError> {
        debug!("Face {:?}", p);
        // TODO(wathiede): add support for quad faces, triangles only for now.
        if p.len() != 4 {
            return Err(ObjectError {
                desc: "Bad vertex line",
                cause: ErrorRepr::ParseError(p.join(" ")),
            });
        };
        let mut f = FaceIndex::new();
        for n in p.iter().skip(1) {
            for (i, c) in n.split("/").enumerate() {
                // Face indices in wavefront object files are 1-based.
                let idx = try!(c.parse::<usize>()) - 1;
                match (i, idx) {
                    (0, idx) => f.v_idxs.push(idx),
                    (1, idx) => f.t_idxs.push(idx),
                    (2, idx) => f.n_idxs.push(idx),
                    (_, _) => panic!("Found more than 3 components in face"),
                }
            }
        }
        self.faces.push(f);
        Ok(())
    }

    fn parse_vec3f(p: Vec<&str>) -> Result<Vec3f, ObjectError> {
        if p.len() != 4 {
            return Err(ObjectError {
                desc: "Bad line", /* desc: format!("Got {} vert components, expected 4: {:?}", p.len(), p).to_string(), */
                cause: ErrorRepr::ParseError(p.join(" ")),
            });
        };
        let x = try!(p[1].parse::<f32>());
        let y = try!(p[2].parse::<f32>());
        let z = try!(p[3].parse::<f32>());
        Ok(Vertex { x: x, y: y, z: z })
    }

    fn add_vertex(&mut self, p: Vec<&str>) -> Result<(), ObjectError> {
        debug!("Vertex {:?}", p);
        // "v <x> <y> <z>"
        let v = try!(Object::parse_vec3f(p));
        self.vertices.push(v);
        Ok(())
    }
    fn add_texcoord(&mut self, p: Vec<&str>) -> Result<(), ObjectError> {
        debug!("Texcoord {:?}", p);
        // "vt <x> <y> <z>"
        let v = try!(Object::parse_vec3f(p));
        self.texcoords.push(v);
        Ok(())
    }
    fn add_normal(&mut self, p: Vec<&str>) -> Result<(), ObjectError> {
        debug!("Normal {:?}", p);
        // "vn <x> <y> <z>"
        let v = try!(Object::parse_vec3f(p));
        self.normals.push(v);
        Ok(())
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f,
               "{} vertices {} faces",
               self.vertices.len(),
               self.faces.len())
    }
}

pub struct ObjectIter<'a> {
    obj: &'a Object,
    idx: usize,
}

impl<'a> iter::Iterator for ObjectIter<'a> {
    type Item = Face;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.obj.faces.len() {
            return None;
        }
        let ref f_idx = self.obj.faces[self.idx];
        // TODO(wathiede): add texcoord/normal values.
        let face = Face {
            vertices: [self.obj.vertex(f_idx.v_idxs[0]),
                       self.obj.vertex(f_idx.v_idxs[1]),
                       self.obj.vertex(f_idx.v_idxs[2])],
            texcoords: [self.obj.texcoord(f_idx.t_idxs[0]),
                        self.obj.texcoord(f_idx.t_idxs[1]),
                        self.obj.texcoord(f_idx.t_idxs[2])],
            normals: [self.obj.normal(f_idx.n_idxs[0]),
                      self.obj.normal(f_idx.n_idxs[1]),
                      self.obj.normal(f_idx.n_idxs[2])],
        };
        self.idx += 1;
        Some(face)
    }
}

impl<'a> iter::IntoIterator for &'a Object {
    type Item = Face;
    type IntoIter = ObjectIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ObjectIter {
            obj: self,
            idx: 0,
        }
    }
}
