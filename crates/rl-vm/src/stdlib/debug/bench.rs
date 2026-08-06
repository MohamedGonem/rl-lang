use crate::{
    stdlib::macros::{verr, vf, vok, vs},
    values::VmValue,
    vm_logic::Vm,
};

pub fn func(eval: &mut Vm, function: VmValue, iterations_val: VmValue) -> VmValue {
    if !matches!(
        &function,
        VmValue::Function { .. } | VmValue::Native(_) | VmValue::Closure { .. }
    ) {
        return verr!(vs!(format!(
            "bench: expects a function or lambda, got {}",
            function.type_name()
        )));
    }

    let iterations = match iterations_val {
        VmValue::Int(n) if n > 0 => n as u64,
        other => {
            return verr!(vs!(format!(
                "bench: expects a positive int for iterations, got {}",
                other.type_name()
            )));
        }
    };

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        if let Err(e) = eval.call_value(&function, &[], eval.current_span()) {
            return verr!(vs!(format!(
                "bench: error executing the function: {}",
                e.message()
            )));
        };
    }
    let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;

    vok!(vf!(elapsed_ms))
}
