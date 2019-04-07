use super::ast::TranscodeAST;
use super::Image;
use serde_json::Value;
use base64::encode;

fn to_dynamic_image(orig: Image) -> image::DynamicImage {
    match orig {
        Image::Rgb(buf) => image::DynamicImage::ImageRgb8(buf),
        Image::Rgba(buf) => image::DynamicImage::ImageRgba8(buf),
        Image::Gray(buf) => image::DynamicImage::ImageLuma8(buf),
        Image::GrayAlpha(buf) => image::DynamicImage::ImageLumaA8(buf)
    }
}

fn image_to_b64(img: Image) -> String {
    let dynamic_image = to_dynamic_image(img);
    let mut buffer = Vec::new();
    dynamic_image.write_to(&mut buffer, image::ImageOutputFormat::PNG).unwrap();
    format!("data:image/png;base64,{}", encode(&buffer))
}

pub fn print(context: Vec<TranscodeAST>) -> String {
    let mut values: Vec<Value> = vec![];

    for ast in context.into_iter() {
        let value = match ast {
            TranscodeAST::Difference { left_pc, right_pc, result } => json!({
                "_type": "difference",
                "left_pc": left_pc,
                "right_pc": right_pc,
                "result": result.map(|img| image_to_b64(img)).unwrap_or(String::new())
            }),
            TranscodeAST::Threshold { target_pc, thresh, result } => json!({
                "_type": "threshold",
                "target_pc": target_pc,
                "thresh": thresh,
                "result": result.map(|img| image_to_b64(img)).unwrap_or(String::new())
            }),
            TranscodeAST::Grayscale { target_pc, result } => json!({
                "_type": "grayscale",
                "target_pc": target_pc,
                "result": result.map(|img| image_to_b64(img)).unwrap_or(String::new())
            }),
            TranscodeAST::MorphologyErode { target_pc, kernel, result } => json!({
                "_type": "morphology_erode",
                "target_pc": target_pc,
                "kernel": json!({
                    "_type": "disk", // FIXME!
                    "radius": kernel.width/2
                }),
                "result": result.map(|img| image_to_b64(img)).unwrap_or(String::new())
            }),
            TranscodeAST::MorphologyDilate { target_pc, kernel, result } => json!({
                "_type": "morphology_dilate",
                "target_pc": target_pc,
                "kernel": json!({
                    "_type": "disk", // FIXME!
                    "radius": kernel.width/2
                }),
                "result": result.map(|img| image_to_b64(img)).unwrap_or(String::new())
            }),
            TranscodeAST::Image { data } => json!({
                "_type": "image",
                "data": image_to_b64(data)
            })
        };
        values.push(value);
    }

    Value::Array(values).to_string()
}