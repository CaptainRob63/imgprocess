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

        let signature: Vec<u8> = next_n(&mut file_iter, 2).ok_or(PpmError::UnexpectedEOF)?;
        if signature.as_slice() != [b'P', b'6'] {
            return Err(PpmError::IncorrectSignature);
        }


        let width = parse_ws_delim_ascii_decimal(&mut file_iter,"whitespace not found after signature")?;

        let height = parse_ws_delim_ascii_decimal(&mut file_iter,"whitespace not found after width")?;

        let maxval = parse_ws_delim_ascii_decimal(&mut file_iter,"whitespace not found after height")?;
        if !(1..65535).contains(&maxval) {
            return Err(PpmError::IllegalValue(maxval, "1..65535".into()));
        }

        if !file_iter.next().ok_or(PpmError::UnexpectedEOF)?.is_ascii_whitespace() {
            return Err(PpmError::IncorrectFormat("whitespace not found after maxval".into()));
        }

        for i in 0..height {
            for j in 0..width {
                if (maxval < 256) {
                    read_byte();
                } else {
                    read_two_byte();
                }
            }
        }

        Ok(PpmImage {
            width,
            height,
            maxval: maxval as u16,
            raster: vec![0],
        })
    }
}

fn parse_ws_delim_ascii_decimal<I>(iterator: &mut I, incorrect_ws_msg: &'static str ) -> Result<u32, PpmError>
where
    I: Iterator<Item = u8>,
{
    let mut next_byte = iterator
        .next()
        .ok_or(PpmError::UnexpectedEOF)?;

    if !next_byte.is_ascii_whitespace() {
        return Err(PpmError::IncorrectFormat(incorrect_ws_msg.to_string()));
    }

    while iterator.next().ok_or(PpmError::UnexpectedEOF)?.is_ascii_whitespace() {
        // skip whitespace
    }

    let mut buffer = vec![];
    while !next_byte.is_ascii_whitespace() {
        buffer.push(next_byte);
        next_byte = iterator.next().ok_or(PpmError::UnexpectedEOF)?;
    }

    Ok(String::from_utf8(buffer)?.parse::<u32>()?)
}

fn next_n<I>(iterator: &mut I, n: u32) -> Option<Vec<I::Item>>
where
    I: Iterator,
{
    let mut vec = vec![];
    for i in 1..n {
        vec.push(iterator.next()?);
    }
    Some(vec)
}

#[derive(Error, Debug)]
pub enum PpmError {
    #[error("Ppm image signature incorrect")]
    IncorrectSignature,

    #[error("Ppm image incorrect format: {}", .0)]
    IncorrectFormat(String),

    #[error("Ppm image illegal value: {}, should be in {}", .0, .1)]
    IllegalValue(u32, String),

    #[error("Ppm image io error: {}", .0)]
    Io(#[from] std::io::Error),

    #[error("Ppm image string conversion error: {}", .0)]
    StringConversion(#[from] std::string::FromUtf8Error),

    #[error("Ppm image parse error: {}", .0)]
    Parse(#[from] std::num::ParseIntError),

    #[error("Ppm image file ends unexpectedly")]
    UnexpectedEOF,
}
