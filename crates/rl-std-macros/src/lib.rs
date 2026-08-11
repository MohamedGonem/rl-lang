//! Procedural macros for the rl-lang standard library.
//!
//! `#[native_fn(...)]` lowers an annotated stdlib function into:
//! - a generic thin-`fn`-pointer wrapper (`wrapper::<R>`) that arity-checks,
//!   extracts each argument, calls the body, and converts the result,
//! - a `signature()` builder producing the checker's [`StdFn`],
//! - a `handle::<R>()` builder producing a [`NativeHandle`].
//!
//! The three are placed in a `mod <fn-name>` sitting beside the (unchanged)
//! function, so `mod::handle::<R>()` / `mod::signature()` can be aggregated by
//! the module builders. See the crate-level docs in `rl-std` for usage.
//!
//! ## Argument classification
//! Each Rust parameter is classified structurally:
//! - a leading `&mut R::Cx` -> the runtime context (forwarded, not an rl arg),
//! - `R::Value` -> a raw value argument (identity extraction),
//! - `Vec<R::Value>` -> variadic (the whole argument vector),
//! - `R::Span` -> the call span,
//! - anything else -> a typed argument extracted via `FromValueR`.
//!
//! ## Return classification
//! - `Result<T, Error>` -> fallible; the error propagates, sig return is `T`,
//! - `Result<T, String>` -> a language `result[T]` value (Ok/Err),
//! - `R::Value` -> a raw value (requires an explicit `sig`),
//! - anything else -> converted via `IntoValueR`, sig derived from the type.

mod attr;
mod codegen;

use proc_macro::TokenStream;

/// Attribute macro: see the crate-level documentation.
#[proc_macro_attribute]
pub fn native_fn(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = syn::parse_macro_input!(attr as attr::NativeFnAttr);
    let func = syn::parse_macro_input!(item as syn::ItemFn);
    match codegen::expand(attr, func) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}
