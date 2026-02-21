mod ppmimage;
use crate::image::ppmimage::PpmImage;

pub enum Image {
    Ppm(PpmImage),
}
