extern crate image;
extern crate imageproc;
extern crate itertools;

#[derive(Clone)]
pub enum Image {
    Gray(image::GrayImage),
    GrayAlpha(image::GrayAlphaImage),
    Rgb(image::RgbImage),
    Rgba(image::RgbaImage)
}

pub mod procedures;