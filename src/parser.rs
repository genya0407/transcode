use super::ast::TranscodeAST;
use super::Kernel;
use super::Image;
use simple_error::SimpleError;
use std::error::Error;
use serde_json::Value;
use base64::decode;

fn pe() -> SimpleError {
    SimpleError::new("JSON parse failed")
}

fn from_dynamic_image(orig: image::DynamicImage) -> Result<Image, Box<Error>> {
    match orig {
        image::DynamicImage::ImageRgb8(buf) => Ok(Image::Rgb(buf)),
        image::DynamicImage::ImageRgba8(buf) => Ok(Image::Rgba(buf)),
        image::DynamicImage::ImageLuma8(buf) => Ok(Image::Gray(buf)),
        image::DynamicImage::ImageLumaA8(buf) => Ok(Image::GrayAlpha(buf)),
        _ => Err(Box::new(pe()))
    }
}

pub fn parse_json(json_string: String) -> Result<Vec<TranscodeAST>, Box<Error>> {
    let mut context = vec![];
    let value: Value = serde_json::from_str(&json_string)?;

    for ast_record in value.as_array().ok_or(pe())? {
        match ast_record["_type"].as_str().ok_or(pe())? {
            "difference" => {
                let left_pc = ast_record["left_pc"].as_u64().ok_or(pe())? as usize;
                let right_pc = ast_record["right_pc"].as_u64().ok_or(pe())? as usize;
                context.push(TranscodeAST::Difference { left_pc: left_pc, right_pc: right_pc, result: None });
            },
            "threshold" => {
                let target_pc = ast_record["target_pc"].as_u64().ok_or(pe())? as usize;
                let thresh = ast_record["thresh"].as_u64().ok_or(pe())? as u8;
                context.push(TranscodeAST::Threshold { target_pc: target_pc, thresh: thresh, result: None });
            },
            "grayscale" => {
                let target_pc = ast_record["target_pc"].as_u64().ok_or(pe())? as usize;
                context.push(TranscodeAST::Grayscale { target_pc: target_pc, result: None });
            },
            "morphology_erode" => {
                let target_pc = ast_record["target_pc"].as_u64().ok_or(pe())? as usize;
                let kernel = ast_record["kernel"].as_object().ok_or(pe())?;
                let kernel = match kernel.get("_type").ok_or(pe())?.as_str().ok_or(pe())? {
                    "disk" => Kernel::disk(kernel.get("radius").ok_or(pe())?.as_u64().ok_or(pe())? as u32),
                    _ => return Err(Box::new(pe()))
                };
                context.push(TranscodeAST::MorphologyErode { target_pc: target_pc, kernel: kernel, result: None });
            },
            "morphology_dilate" => {
                let target_pc = ast_record["target_pc"].as_u64().ok_or(pe())? as usize;
                let kernel = ast_record["kernel"].as_object().ok_or(pe())?;
                let kernel = match kernel.get("_type").ok_or(pe())?.as_str().ok_or(pe())? {
                    "disk" => Kernel::disk(kernel.get("radius").ok_or(pe())?.as_u64().ok_or(pe())? as u32),
                    _ => return Err(Box::new(pe()))
                };
                context.push(TranscodeAST::MorphologyDilate { target_pc: target_pc, kernel: kernel, result: None });
            },
            "image" => {
                let data = decode(ast_record["data"].as_str().ok_or(pe())?).map_err(|_| pe())?;
                let format = image::guess_format(&data).map_err(|_| pe())?;
                let file = std::io::BufReader::new(std::io::Cursor::new(data));
                let img = from_dynamic_image(image::load(file, format).map_err(|_| pe())?)?;
                context.push(TranscodeAST::Image { data: img });
            },
            _ => panic!("Not implemented!")
        }
    }

    return Ok(context);
}