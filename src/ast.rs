use super::{Image, Kernel};

pub enum TranscodeAST {
    Difference { left: Box<TranscodeAST>, right: Box<TranscodeAST>, result: Option<Image> },
    Threshold { target: Box<TranscodeAST>, thresh: u8, result: Option<Image> },
    Grayscale { target: Box<TranscodeAST>, result: Option<Image> },
    MorphologyErode { target: Box<TranscodeAST>, kernel: Kernel, result: Option<Image> },
    MorphologyDilate { target: Box<TranscodeAST>, kernel: Kernel, result: Option<Image> },
    Image { data: Image }
}

impl TranscodeAST {
    pub fn result_image(&self) -> Image {
        match self {
            TranscodeAST::Difference { result, .. } => result.clone().unwrap(),
            TranscodeAST::Threshold { result, .. } => result.clone().unwrap(),
            TranscodeAST::Grayscale { result, .. } => result.clone().unwrap(),
            TranscodeAST::MorphologyErode { result, .. } => result.clone().unwrap(),
            TranscodeAST::MorphologyDilate { result, .. } => result.clone().unwrap(),
            TranscodeAST::Image { data, .. } => data.clone()
        }
    }
}