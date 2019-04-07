extern crate transcode;
extern crate image;

use transcode::ast::TranscodeAST;
use transcode::Kernel;

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

    let context = vec![
        TranscodeAST::Image { data: img },
        TranscodeAST::Grayscale { target_pc: 0, result: None },
        TranscodeAST::MorphologyErode { target_pc: 1, kernel: Kernel::disk(10), result: None },
        TranscodeAST::MorphologyDilate { target_pc: 2, kernel: Kernel::disk(10), result: None },
        TranscodeAST::Difference { left_pc: 1, right_pc: 3, result: None },
        TranscodeAST::Threshold { target_pc: 4, thresh: 50, result: None }
    ];
    let mut evaluator = transcode::evaluator::Evaluator { context: context };
    evaluator.run();
    for (index, ast) in evaluator.context.into_iter().enumerate() {
        save(&format!("./progress/{:0width$}.png", index, width = 4), &ast.result_image());
    }
}
