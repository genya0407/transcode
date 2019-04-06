extern crate transcode;
extern crate image;

fn main() {
    let img = image::open("test_img/test_01.jpg").unwrap();
    let img = match img {
        image::DynamicImage::ImageRgb8(buf) => transcode::Image::Rgb(buf),
        _ => panic!("not rgb!")
    };
    let gray = transcode::grayscale(img).unwrap();
    let bin = transcode::threshold(gray.clone(), 100).unwrap();
    let img = transcode::difference(gray, bin).unwrap();
    match img {
        transcode::Image::Gray(img) => img.save("hoge.png").unwrap(),
        _ => panic!("not rgb!")
    }
}
