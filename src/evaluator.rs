use super::ast::TranscodeAST;
use super::procedures;

pub fn eval_ast(ast: Box<TranscodeAST>) -> Box<TranscodeAST> {
    match *ast {
        TranscodeAST::Difference { left, right, .. } => {
            let left = eval_ast(left);
            let right = eval_ast(right);
            let result = procedures::difference(left.result_image(), right.result_image()).unwrap();
            Box::new(TranscodeAST::Difference { left: left, right: right, result: Some(result) })
        },
        x => Box::new(x)
    }
}