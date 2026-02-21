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
        let mut file = vec![];
        reader.read_to_end(&mut file)?;
        let mut file_iter = file.into_iter();

        let signature: Vec<u8> = file_iter.by_ref().take(2).collect();

        if signature.as_slice() != [b'P', b'6'] {
            return Err(PpmError::IncorrectSignature);
        }

        while file_iter
            .next()
            .ok_or(PpmError::UnexpectedEOF)?
            .is_ascii_whitespace()
        {}

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

    #[error("Ppm image file ends unexpectedly")]
    UnexpectedEOF,
}
