extern crate transcode;
extern crate image;

fn from_dynamic_image(orig: image::DynamicImage) -> transcode::Image {
    match orig {
        image::DynamicImage::ImageRgb8(buf) => transcode::Image::Rgb(buf),
        image::DynamicImage::ImageRgba8(buf) => transcode::Image::Rgba(buf),
        image::DynamicImage::ImageLuma8(buf) => transcode::Image::Gray(buf),
        image::DynamicImage::ImageLumaA8(buf) => transcode::Image::GrayAlpha(buf),
        _ => panic!("unsupported image type.")
    }
}

fn save(path: &str, img: &transcode::Image) {
    match img {
        transcode::Image::Rgb(img) => img.save(path).unwrap(),
        transcode::Image::Rgba(img) => img.save(path).unwrap(),
        transcode::Image::Gray(img) => img.save(path).unwrap(),
        transcode::Image::GrayAlpha(img) => img.save(path).unwrap()
    }
}

fn kernel(width: u32, height: u32) -> image::GrayImage {
    image::GrayImage::from_raw(width, height, vec![0; (width * height) as usize]).unwrap()
}

fn main() {
    let img = image::open("test_img/test_03.png").unwrap();

    let img = from_dynamic_image(img);
    let gray = transcode::grayscale(img).unwrap();
    save("./progress/gray.png", &gray);

    let eroded = transcode::morphology_erode(gray, transcode::MorphologyErodeOpts{kernel: kernel(15,15)}).unwrap();
    save("./progress/eroded.png", &eroded);
}
