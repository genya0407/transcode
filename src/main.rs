extern crate transcode;
extern crate image;

use transcode::ast::TranscodeAST;

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

fn main() {
    let img = image::open("test_img/test_03.png").unwrap();

    let img = from_dynamic_image(img);

    let gray = transcode::procedures::grayscale(img).unwrap();
    save("./progress/gray.png", &gray);

    let eroded = transcode::procedures::morphology_erode(gray.clone(), transcode::Kernel::disk(10)).unwrap();
    save("./progress/eroded.png", &eroded);

    let dilated = transcode::procedures::morphology_dilate(eroded, transcode::Kernel::disk(10)).unwrap();
    save("./progress/dilated.png", &dilated);

    //let flattened = transcode::procedures::difference(gray, dilated).unwrap();
    let ast = TranscodeAST::Difference {
        left: Box::new(TranscodeAST::Image{ data: gray }),
        right: Box::new(TranscodeAST::Image { data: dilated }),
        result: None
    };
    let flattened = transcode::evaluator::eval_ast(Box::new(ast)).result_image();

    save("./progress/flattened.png", &flattened);
}
