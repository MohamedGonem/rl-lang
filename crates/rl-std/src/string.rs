//! `std::str` - string manipulation.
//!
//! The rl module name is `str`; the Rust module is `string` to avoid clashing
//! with the `str` primitive. Ported once from the former per-runtime
//! `stdlib/string/*.rs` copies.

use rl_std_core::Runtime;
use rl_std_macros::native_fn;
use rl_utils::errors::Error;

// ---- simple `string -> string` transforms ---------------------------------

#[native_fn(module = "str")]
pub fn to_upper(string: String) -> String {
    string.to_uppercase()
}

#[native_fn(module = "str")]
pub fn to_lower(string: String) -> String {
    string.to_lowercase()
}

#[native_fn(module = "str")]
pub fn trim(string: String) -> String {
    string.trim().to_string()
}

#[native_fn(module = "str")]
pub fn trim_end(string: String) -> String {
    string.trim_end().to_string()
}

#[native_fn(module = "str")]
pub fn trim_start(string: String) -> String {
    string.trim_start().to_string()
}

#[native_fn(module = "str")]
pub fn reverse(string: String) -> String {
    string.chars().rev().collect()
}

// ---- scalar queries / transforms ------------------------------------------

#[native_fn(module = "str")]
pub fn repeat(string: String, count: i64) -> String {
    string.repeat(count as usize)
}

#[native_fn(module = "str")]
pub fn is_empty(string: String) -> bool {
    string.is_empty()
}

#[native_fn(module = "str")]
pub fn contains(string: String, sub: String) -> bool {
    string.contains(&sub)
}

#[native_fn(module = "str")]
pub fn starts_with(string: String, sub: String) -> bool {
    string.starts_with(&sub)
}

#[native_fn(module = "str")]
pub fn ends_with(string: String, sub: String) -> bool {
    string.ends_with(&sub)
}

#[native_fn(module = "str")]
pub fn replace(string: String, from: String, to: String) -> String {
    string.replace(&from, &to)
}

#[native_fn(module = "str")]
pub fn pad_left(string: String, width: i64, character: char) -> String {
    let pad = (width as usize).saturating_sub(string.chars().count());
    format!("{}{}", character.to_string().repeat(pad), string)
}

#[native_fn(module = "str")]
pub fn pad_right(string: String, width: i64, character: char) -> String {
    let pad = (width as usize).saturating_sub(string.chars().count());
    format!("{}{}", string, character.to_string().repeat(pad))
}

#[native_fn(module = "str")]
pub fn count(string: String, to_count: String) -> i64 {
    string.matches(&to_count).count() as i64
}

#[native_fn(module = "str")]
pub fn index_of(string: String, sub: String) -> i64 {
    match string.find(&sub) {
        Some(i) => string[..i].chars().count() as i64,
        None => -1_i64,
    }
}

// ---- array-producing ------------------------------------------------------

#[native_fn(module = "str")]
pub fn bytes(string: String) -> Vec<u8> {
    string.bytes().collect()
}

#[native_fn(module = "str")]
pub fn chars(string: String) -> Vec<char> {
    string.chars().collect()
}

#[native_fn(module = "str")]
pub fn split(string: String, delim: String) -> Vec<String> {
    string.split(&delim).map(|s| s.to_string()).collect()
}

// ---- fallible (language `result[T]`) --------------------------------------

#[native_fn(module = "str")]
pub fn char_at(string: String, index: i64) -> Result<char, String> {
    if index < 0 {
        return Err(format!("index cannot be negative: {index}"));
    }
    let mut chars = string.chars();
    let chars_count = chars.clone().count();
    if index as usize >= chars_count {
        Err(format!(
            "index out of bounds string length is {chars_count} , used {index}"
        ))
    } else {
        Ok(chars.nth(index as usize).unwrap())
    }
}

#[native_fn(module = "str")]
pub fn slice(string: String, start: i64, end: i64) -> Result<String, String> {
    let chars = string.chars();
    let chars_count = chars.clone().count();
    if start as usize >= chars_count || end as usize > chars_count {
        return Err(format!(
            "index out of bounds string legth: {chars_count}, found start: {start} and end: {end}"
        ));
    }
    Ok(chars
        .skip(start as usize)
        .take(end as usize - start as usize)
        .collect::<String>())
}

// `array[_]` first arg: any element type, stringified. Explicit signature
// because the argument is a raw runtime value.
#[native_fn(module = "str", sig(array[_], string -> result[string]))]
pub fn join<R: Runtime>(_cx: &mut R::Cx, array: R::Value, delim: String) -> Result<String, String> {
    let Some((items, _)) = R::as_array(&array) else {
        return Err("join() expects an array as first argument".to_string());
    };
    let mut parts: Vec<String> = Vec::with_capacity(items.len());
    for v in items {
        if R::is_callable(v) {
            return Err("functions/lambdas/enclosures are not supported via join()".to_string());
        }
        parts.push(R::display(v));
    }
    Ok(parts.join(&delim))
}

// ---- variadic -------------------------------------------------------------

#[native_fn(module = "str", untyped)]
pub fn concat<R: Runtime>(args: Vec<R::Value>) -> String {
    args.iter().map(|v| R::display(v)).collect()
}

#[native_fn(module = "str", untyped)]
pub fn format<R: Runtime>(cx: &mut R::Cx, args: Vec<R::Value>, span: R::Span) -> Result<String, Error> {
    if args.is_empty() {
        return Err(R::error(cx, "expected arguments", span));
    }

    let args: Vec<String> = args.iter().map(|v| R::display(v)).collect();
    let text = &args[0];
    let mut rest_args = args[1..].iter();
    let mut chars = text.chars().peekable();
    let mut result = String::new();
    let mut used = 0;
    let mut missing = 0;

    while let Some(c) = chars.next() {
        if c == '{' && chars.peek() == Some(&'}') {
            chars.next();
            match rest_args.next() {
                Some(v) => {
                    result.push_str(v);
                    used += 1;
                }
                None => {
                    result.push_str("{}");
                    missing += 1;
                }
            }
        } else {
            result.push(c);
        }
    }

    if missing > 0 {
        return Err(R::error(
            cx,
            format!("format() has {missing} placeholder(s) with no matching argument"),
            span,
        ));
    }
    if used < args.len() - 1 {
        return Err(R::error(
            cx,
            format!(
                "format() received {} argument(s) but only {} placeholder(s) were used",
                args.len() - 1,
                used
            ),
            span,
        ));
    }

    Ok(result)
}

rl_std_core::native_module!("str";
    funcs: [
        to_upper, to_lower, trim, trim_end, trim_start, reverse,
        repeat, is_empty, contains, starts_with, ends_with, replace,
        pad_left, pad_right, count, index_of,
        bytes, chars, split,
        char_at, slice, join,
        concat, format,
    ],
);
