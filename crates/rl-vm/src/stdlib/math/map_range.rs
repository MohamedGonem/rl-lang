use crate::Vm;

pub fn std_map_range(
    _: &mut Vm,
    value: f64,
    in_min: f64,
    out_min: f64,
    in_max: f64,
    out_max: f64,
) -> f64 {
    (value - in_min) / (in_max - in_min) * (out_max - out_min) + out_min
}
