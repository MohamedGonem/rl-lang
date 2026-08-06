use crate::{evaluator::Evaluator, stdlib::common::vnl, values::Value};

pub fn func(eval: &mut Evaluator) -> Value {
    eval.gui_quit_requested = true;
    vnl!()
}
