use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::BufReader;

pub struct PpmImage {
    width: u32,
    height: u32,
    maxval: u16,
    raster: Vec<u8>,
}

impl PpmImage {
    pub fn new(width:u32, height:u32, maxval:u16, raster:Vec<u8>) -> PpmImage {
        PpmImage {
            width,
            height,
            maxval,
            raster,
        }
    }

    pub fn from_file(file: File) -> Result<PpmImage, PpmError> {
        let reader = BufReader::new(file);
        Ok(PpmImage {
            width: 0,
            height: 0,
            maxval: 0,
            raster: vec!(0)
        })

    }

}

#[derive(Debug)]
pub struct PpmError {
    message: String,
}


impl fmt::Display for PpmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for PpmError {}