extern crate transcode;
extern crate image;

fn main() {
    let img = image::open("test_img/test_01.jpg").unwrap();
    let img = match img {
        image::DynamicImage::ImageRgb8(buf) => transcode::Image::Rgb(buf),
        _ => panic!("not rgb!")
    };
    let img = transcode::difference(img.clone(), img).unwrap();
    match img {
        transcode::Image::Rgb(img) => img.save("hoge.png").unwrap(),
        _ => panic!("not rgb!")
    }
}
