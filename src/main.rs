extern crate transcode;
extern crate image;

use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let context = transcode::parser::parse_json(buffer).unwrap();
    let mut evaluator = transcode::evaluator::Evaluator { context: context };
    evaluator.run();
    for (index, ast) in evaluator.context.into_iter().enumerate() {
        save(&format!("./progress/{:0width$}.png", index, width = 4), &ast.result_image());
    }
    Ok(())
}

fn save(path: &str, img: &transcode::Image) {
    match img {
        transcode::Image::Rgb(img) => img.save(path).unwrap(),
        transcode::Image::Rgba(img) => img.save(path).unwrap(),
        transcode::Image::Gray(img) => img.save(path).unwrap(),
        transcode::Image::GrayAlpha(img) => img.save(path).unwrap()
    }
}
