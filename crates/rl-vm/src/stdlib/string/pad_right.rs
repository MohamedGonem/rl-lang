use crate::Vm;

pub fn std_pad_right(_: &mut Vm, string: String, width: i64, character: char) -> String {
    let pad = (width as usize).saturating_sub(string.chars().count());
    format!("{}{}", string, character.to_string().repeat(pad))
}
