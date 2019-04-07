use super::{Image, Kernel};

#[derive(Clone)]
pub enum TranscodeAST {
    Difference { left_pc: usize, right_pc: usize, result: Option<Image> },
    Threshold { target_pc: usize, thresh: u8, result: Option<Image> },
    Grayscale { target_pc: usize, result: Option<Image> },
    MorphologyErode { target_pc: usize, kernel: Kernel, result: Option<Image> },
    MorphologyDilate { target_pc: usize, kernel: Kernel, result: Option<Image> },
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