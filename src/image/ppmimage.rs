use std::fmt;
use std::fs::File;
use std::io::{BufReader, Read};

use thiserror::Error;

pub struct PpmImage {
    width: u32,
    height: u32,
    maxval: u16,
    raster: Vec<u8>,
}

impl PpmImage {
    pub fn new(width: u32, height: u32, maxval: u16, raster: Vec<u8>) -> PpmImage {
        PpmImage {
            width,
            height,
            maxval,
            raster,
        }
    }

    pub fn from_file(file: File) -> Result<PpmImage, PpmError> {
        let mut reader = BufReader::new(file);
        let mut bytes = vec![];
        reader.read_to_end(&mut bytes);

        let mut buffer = vec![];

        buffer.extend(bytes.take(2));

        if buffer.as_slice() != [b'P', b'6'] {}

        Ok(PpmImage {
            width: 0,
            height: 0,
            maxval: 0,
            raster: vec![0],
        })
    }
}

#[derive(Error, Debug)]
pub enum PpmError {
    #[error("Ppm image signature incorrect")]
    IncorrectSignature,
    #[error("Ppm image io error: {}", .0)]
    Io(#[from] std::io::Error),
}
