use super::ast::TranscodeAST;
use super::procedures;
use super::Image;

pub struct Evaluator {
    pub context: Vec<TranscodeAST>
}

impl Evaluator {
    pub fn run(&mut self) -> Image {
        self.eval(self.context.len()-1);
        self.context.last().unwrap().result_image()
    }

    pub fn eval(&mut self, pc: usize) {
        match self.context[pc].clone() {
            TranscodeAST::Difference { left_pc, right_pc, result } => {
                if result.is_none() {
                    self.eval(left_pc);
                    self.eval(right_pc);
                    let result = procedures::difference(
                        self.context[left_pc].result_image(),
                        self.context[right_pc].result_image()
                    ).unwrap();
                    self.context[pc] = TranscodeAST::Difference { left_pc: left_pc, right_pc: right_pc, result: Some(result) };
                }
            },
            TranscodeAST::Threshold { target_pc, thresh, result } => {
                if result.is_none() {
                    self.eval(target_pc);
                    self.context[pc] = TranscodeAST::Threshold {
                        target_pc: target_pc, thresh: thresh,
                        result: Some(procedures::threshold(self.context[target_pc].result_image(), thresh).unwrap())
                    };
                }
            },
            TranscodeAST::Grayscale { target_pc, result } => {
                if result.is_none() {
                    self.eval(target_pc);
                    self.context[pc] = TranscodeAST::Grayscale {
                        target_pc: target_pc,
                        result: Some(procedures::grayscale(self.context[target_pc].result_image()).unwrap())
                    };
                }
            },
            TranscodeAST::MorphologyErode { target_pc, kernel, result } => {
                if result.is_none() {
                    self.eval(target_pc);
                    self.context[pc] = TranscodeAST::MorphologyErode {
                        target_pc: target_pc,
                        kernel: kernel.clone(),
                        result: Some(procedures::morphology_erode(self.context[target_pc].result_image(), kernel).unwrap())
                    };
                }
            },
            TranscodeAST::MorphologyDilate { target_pc, kernel, result } => {
                if result.is_none() {
                    self.eval(target_pc);
                    self.context[pc] = TranscodeAST::MorphologyDilate {
                        target_pc: target_pc,
                        kernel: kernel.clone(),
                        result: Some(procedures::morphology_dilate(self.context[target_pc].result_image(), kernel).unwrap())
                    };
                }
            },
            TranscodeAST::Image { .. } => ()
        }
    }
}