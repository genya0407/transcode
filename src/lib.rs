extern crate image;
use image::ImageBuffer;

#[derive(Clone)]
pub enum Image {
    Gray(image::GrayImage),
    GrayAlpha(image::GrayAlphaImage),
    Rgb(image::RgbImage),
    Rgba(image::RgbaImage)
}

#[derive(Debug)]
pub enum ImageTypeError {
    ColorTypeMismatched
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

fn _difference<P: image::Pixel + 'static>(left: ImageBuffer<P, Vec<P::Subpixel>>, right: ImageBuffer<P, Vec<P::Subpixel>>)
    -> ImageBuffer<P, Vec<P::Subpixel>> {
    let width: u32 = left.width();
    let height: u32 = left.height();
    let result_raw = left.into_vec().into_iter().zip(right.into_vec().into_iter()).map(|(l, r)| l - r).collect();
    return ImageBuffer::from_raw(width, height, result_raw).unwrap();
}