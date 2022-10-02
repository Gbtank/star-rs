use std::fs;
use std::path::{Path};

pub struct Shader {
    pub src: String,
}

impl Shader {
    pub fn from<P: AsRef<Path>>(path: P) -> Self {
        let src = match fs::read_to_string(path) {
            Ok(src) => src,
            Err(err) => {
                panic!("Error attempting to read shader source: {}", err);
            }
        };
        Shader {
            src,
        }
    }
}