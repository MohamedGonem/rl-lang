use crate::{Vm, stdlib::macros::vnl, values::VmValue};

pub fn func(eval: &mut Vm) -> VmValue {
    eval.gui_quit_requested = true;
    vnl!()
}
