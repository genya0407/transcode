extern crate image;
extern crate imageproc;
extern crate itertools;

use image::ImageBuffer;
use itertools::Itertools;

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
pub enum ImageTypeError {
    ColorTypeMismatched
}

pub fn threshold(img: Image, thresh: u8) -> Result<Image, ImageTypeError> {
    match img {
        Image::Gray(img) => Ok(Image::Gray(imageproc::contrast::threshold(&img, thresh))),
        _ => Err(ImageTypeError::ColorTypeMismatched)
    }
}

pub fn grayscale(img: Image) -> Result<Image, ImageTypeError> {
    match img {
        Image::Gray(img)      => Ok(Image::Gray(img)),
        Image::GrayAlpha(img) => Ok(Image::GrayAlpha(img)),
        Image::Rgb(img)       => Ok(Image::Gray(image::imageops::colorops::grayscale(&img))),
        Image::Rgba(img)      => Ok(Image::Gray(image::imageops::colorops::grayscale(&img)))
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

pub struct MorphologyErodeOpts {
    pub kernel: Kernel
}

pub fn morphology_erode(img: Image, opts: MorphologyErodeOpts) -> Result<Image, ImageTypeError> {
    match img {
        Image::Gray(original) => {
            let kernel = opts.kernel;
            let mut new_img = image::GrayImage::new(original.width(), original.height());
            for (x, y, p) in new_img.enumerate_pixels_mut() {
                let eroded_pixel = ((-(kernel.width as i64/2))..=(kernel.width as i64/2))
                    .cartesian_product((-(kernel.height as i64/2))..=(kernel.height as i64/2))
                    .map(|(dx, dy)| {
                        let x = (x as i64 + dx) as u32;
                        let y = (y as i64 + dy) as u32;
                        if x < original.width() && y < original.height() {
                            if let Some(kernel_pixel) = kernel.at(dx, dy) {
                                let orig_pixel = original.get_pixel(x, y).data[0];
                                orig_pixel.saturating_sub(kernel_pixel)
                            } else {
                                255 // for ignoring this pixel
                            }
                        } else {
                            255 // for ignoring this pixel
                        }
                    }).min().unwrap();
                *p = image::Luma { data: [eroded_pixel] };
            }
            Ok(Image::Gray(new_img))
        },
        _                => Err(ImageTypeError::ColorTypeMismatched)
    }
}

pub struct MorphologyDilateOpts {
    pub kernel: Kernel
}

pub fn morphology_dilate(img: Image, opts: MorphologyDilateOpts) -> Result<Image, ImageTypeError> {
    match img {
        Image::Gray(original) => {
            let kernel = opts.kernel;
            let mut new_img = image::GrayImage::new(original.width(), original.height());
            for (x, y, p) in new_img.enumerate_pixels_mut() {
                let dilated_pixel = ((-(kernel.width as i64/2))..=(kernel.width as i64/2))
                    .cartesian_product((-(kernel.height as i64/2))..=(kernel.height as i64/2))
                    .map(|(dx, dy)| {
                        let x = (x as i64 + dx) as u32;
                        let y = (y as i64 + dy) as u32;
                        if x < original.width() && y < original.height() {
                            if let Some(kernel_pixel) = kernel.at(dx, dy) {
                                let orig_pixel = original.get_pixel(x, y).data[0];
                                orig_pixel.saturating_add(kernel_pixel)
                            } else {
                                0 // for ignoring this pixel
                            }
                        } else {
                            0 // for ignoring this pixel
                        }
                    }).max().unwrap();
                *p = image::Luma { data: [dilated_pixel] };
            }
            Ok(Image::Gray(new_img))
        },
        _                => Err(ImageTypeError::ColorTypeMismatched)
    }
}

pub fn difference(left: Image, right: Image) -> Result<Image, ImageTypeError> {
    match (left, right) {
        (Image::Gray(left), Image::Gray(right)) => Ok(Image::Gray(_difference(left, right))),
        (Image::GrayAlpha(left), Image::GrayAlpha(right)) => Ok(Image::GrayAlpha(_difference(left, right))),
        (Image::Rgb(left), Image::Rgb(right)) => Ok(Image::Rgb(_difference(left, right))),
        (Image::Rgba(left), Image::Rgba(right)) => Ok(Image::Rgba(_difference(left, right))),
        _ => Err(ImageTypeError::ColorTypeMismatched)
    }
}

fn _difference<P: image::Pixel<Subpixel=u8> + 'static>(left: ImageBuffer<P, Vec<P::Subpixel>>, right: ImageBuffer<P, Vec<P::Subpixel>>)
    -> ImageBuffer<P, Vec<P::Subpixel>> {
    let width: u32 = left.width();
    let height: u32 = left.height();
    let result_raw = left.into_vec().into_iter().zip(right.into_vec().into_iter()).map(|(l, r)| l.saturating_sub(r)).collect();
    return ImageBuffer::from_raw(width, height, result_raw).unwrap();
}