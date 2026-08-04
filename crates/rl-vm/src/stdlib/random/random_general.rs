use crate::{
    Vm,
    stdlib::macros::{vby, verr, vi, vok, vs},
    values::VmValue,
};
use std::rc::Rc;

pub fn dice(eval: &mut Vm, sides: i64) -> VmValue {
    if sides <= 0 {
        return verr!(vs!("sides should be 1 or higher".to_string()));
    }
    vok!(vi!(eval.rng.generate_random_int_range(1, sides)))
}

pub fn dices(eval: &mut Vm, count: i64, sides: i64) -> VmValue {
    if count <= 0 {
        return verr!(vs!("count should be 1 or higher".to_string()));
    }
    if sides <= 0 {
        return verr!(vs!("sides should be 1 or higher".to_string()));
    }

    let result: Vec<VmValue> = (0..count)
        .map(|_| vi!(eval.rng.generate_random_int_range(1, sides)))
        .collect();

    vok!(VmValue::Arr(Rc::new(result,)))
}

pub fn range(eval: &mut Vm, stop: i64) -> VmValue {
    if 0 == stop {
        return verr!(vs!("rand_range() stop shouldn't be zero".to_string()));
    }
    if 0 > stop {
        return verr!(vs!(
            "rand_range() stop shouldn't be less than zero".to_string()
        ));
    }

    vok!(vi!(eval.rng.generate_random_int_range(0, stop)))
}

pub fn range_step(eval: &mut Vm, start: i64, end: i64, step: i64) -> VmValue {
    if 0 == step {
        return verr!(vs!("rand_range_step() stop shouldn't be zero".to_string()));
    }
    if start >= end {
        return verr!(vs!(format!(
            "rand_range_step() end shouldn't be less than or equal to {}",
            start
        )));
    }

    let count = ((end - start) / step) + 1;
    let i = eval.rng.generate_random_int_range(0, count - 1);
    vok!(vi!(start + i * step))
}

pub fn choice(eval: &mut Vm, array: VmValue) -> VmValue {
    match array {
        VmValue::Arr(items) => {
            if items.is_empty() {
                return verr!(vs!("array is empty".to_string()));
            }
            vok!(
                items[eval
                    .rng
                    .generate_random_int_range(0, items.len() as i64 - 1)
                    as usize]
                    .clone()
            )
        }

        other => verr!(vs!(format!("rand_choice() expected array found {}", other))),
    }
}

pub fn choices(eval: &mut Vm, array: VmValue, count: i64) -> VmValue {
    if count <= 0 {
        return verr!(vs!("count should be 1 or higher".to_string()));
    }

    match array.clone() {
        VmValue::Arr(items) => {
            if items.is_empty() {
                return verr!(vs!("array is empty".to_string()));
            }
            let result: Vec<VmValue> = (0..count)
                .map(|_| {
                    items[eval
                        .rng
                        .generate_random_int_range(0, items.len() as i64 - 1)
                        as usize]
                        .clone()
                })
                .collect();
            vok!(VmValue::Arr(Rc::new(result)))
        }

        other => verr!(vs!(format!(
            "rand_choices() expected array found {}",
            other
        ))),
    }
}

pub fn sample(eval: &mut Vm, array: VmValue, count: i64) -> VmValue {
    if count <= 0 {
        return verr!(vs!("count should be 1 or higher".to_string()));
    }

    match array.clone() {
        VmValue::Arr(items) => {
            if count as usize > items.len() {
                return verr!(vs!("count larger than array".to_string()));
            }
            let mut indices: Vec<usize> = (0..items.len()).collect();
            for i in (1..items.len()).rev() {
                let j = eval.rng.generate_random_int_range(0, i as i64) as usize;
                indices.swap(i, j);
            }

            let result: Vec<VmValue> = indices[..count as usize]
                .iter()
                .map(|&i| items[i].clone())
                .collect();

            vok!(VmValue::Arr(Rc::new(result)))
        }

        other => verr!(vs!(format!("rand_sample() expected array found {}", other))),
    }
}

pub fn char(eval: &mut Vm) -> char {
    eval.rng.generate_random_int_range(32, 126) as u8 as char
}

pub fn byte(eval: &mut Vm) -> u8 {
    eval.rng.generate_random_int_range(0, 255) as u8
}

pub fn string(eval: &mut Vm, count: i64) -> VmValue {
    if count <= 0 {
        return verr!(vs!("count cannot be less than or equal to zero".to_string()));
    }

    let result: String = (0..count).map(|_| char(eval)).collect();

    vok!(vs!(result))
}

pub fn bytes(eval: &mut Vm, count: i64) -> VmValue {
    if count <= 0 {
        return verr!(vs!("count cannot be less than zero".to_string()));
    }

    let result: Vec<VmValue> = (0..count).map(|_| vby!(byte(eval))).collect();

    vok!(VmValue::Arr(Rc::new(result,)))
}

pub fn shuffle(eval: &mut Vm, array: VmValue) -> VmValue {
    match array {
        VmValue::Arr(items) => {
            if items.is_empty() {
                return verr!(vs!("array is empty".to_string()));
            }

            let mut items = (*items).clone();
            for i in (1..items.len()).rev() {
                let j = eval.rng.generate_random_int_range(0, i as i64) as usize;
                items.swap(i, j);
            }

            vok!(VmValue::Arr(Rc::new(items)))
        }

        other => verr!(vs!(format!(
            "rand_shuffle() expected array found {}",
            other
        ))),
    }
}
