extern crate image;
extern crate imageproc;
extern crate itertools;

use itertools::Itertools;

pub mod procedures;
pub mod ast;
pub mod evaluator;

#[derive(Clone)]
pub enum Image {
    Gray(image::GrayImage),
    GrayAlpha(image::GrayAlphaImage),
    Rgb(image::RgbImage),
    Rgba(image::RgbaImage)
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Image::Gray(_) => write!(f, "Gray"),
            Image::GrayAlpha(_) => write!(f, "GrayAlpha"),
            Image::Rgb(_) => write!(f, "Rgb"),
            Image::Rgba(_) => write!(f, "Rgba")
        }
    }
}

#[derive(Debug)]
pub struct Kernel {
    width: u32,
    height: u32,
    data: Vec<Option<u8>>
}

impl Kernel {
    // dx, dy: relative position from center.
    pub fn at(&self, dx: i64, dy: i64) -> Option<u8> {
        let x = self.width as i64/2 + dx;
        let y = self.height as i64/2 + dy;
        self.data[(y*self.width as i64 + x) as usize]
    }

    pub fn disk(radius: u32) -> Self {
        let width = radius * 2 + 1;
        let height = width;
        let mut data = vec![None; (width * height) as usize];

        let radius_square = (radius as i64).pow(2);
        let center_x = radius as i64;
        let center_y = radius as i64;
        for (x, y) in (0..width as i64).cartesian_product(0..height as i64) {
            if (x - center_x).pow(2) + (y - center_y).pow(2) <= radius_square {
                data[(y*(width as i64) + x) as usize] = Some(0)
            }
        }

        Self {
            width: width,
            height: height,
            data: data
        }
    }
}
