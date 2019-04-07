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
            TranscodeAST::Difference { left_pc, right_pc, .. } => {
                self.eval(left_pc);
                self.eval(right_pc);
                let result = procedures::difference(
                    self.context[left_pc].result_image(),
                    self.context[right_pc].result_image()
                ).unwrap();
                self.context[pc] = TranscodeAST::Difference { left_pc: left_pc, right_pc: right_pc, result: Some(result) };
            },
            _ => ()
        }
    }
}