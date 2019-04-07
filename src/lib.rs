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

pub struct MorphologyErodeOpts {
    pub kernel: image::GrayImage
}

pub fn morphology_erode(img: Image, opts: MorphologyErodeOpts) -> Result<Image, ImageTypeError> {
    match img {
        Image::Gray(original) => {
            let kernel = opts.kernel;
            let mut new_img = image::GrayImage::new(original.width(), original.height());
            for (x, y, p) in new_img.enumerate_pixels_mut() {
                let eroded_pixel = (0..(kernel.width()))
                    .cartesian_product(0..(kernel.height()))
                    .map(|(dx, dy)| {
                        let x = x+dx;
                        let y = y+dy;
                        if x < original.width() && y < original.height() {
                            let orig_pixel = original.get_pixel(x, y).data[0];
                            let kernel_pixel = kernel.get_pixel(dx, dy).data[0];
                            orig_pixel.saturating_sub(kernel_pixel)
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

pub fn difference(left: Image, right: Image) -> Result<Image, ImageTypeError> {
    println!("{}, {}", left, right);

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