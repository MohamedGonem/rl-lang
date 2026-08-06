//! Parsing for the `#[native_fn(...)]` attribute, including the small
//! signature DSL (`sig(int, int -> result[int])`, `product([byte, int], ... ->
//! result[null])`).

use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitStr, Token, bracketed, parenthesized};

/// One `(params, return)` overload, already lowered to token streams that build
/// `TypeAnnotation` values at runtime.
pub struct Overload {
    pub params: Vec<TokenStream>,
    pub ret: TokenStream,
}

/// The parsed `#[native_fn(...)]` attribute.
pub struct NativeFnAttr {
    pub module: Option<String>,
    pub name: Option<String>,
    /// Explicitly untyped: the checker treats calls as permissive.
    pub untyped: bool,
    /// Explicit overloads (from `sig(...)` / `product(...)`). Empty means
    /// "derive the signature from the Rust types".
    pub overloads: Vec<Overload>,
    /// An extra trait bound to add to the generated `wrapper`/`handle` `R`
    /// generic (e.g. a per-domain handle-store trait like `NetStore`). Parsed
    /// from `bound = "..."`.
    pub bound: Option<TokenStream>,
}

impl Parse for NativeFnAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut out = NativeFnAttr {
            module: None,
            name: None,
            untyped: false,
            overloads: Vec::new(),
            bound: None,
        };

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            match key.to_string().as_str() {
                "module" => {
                    input.parse::<Token![=]>()?;
                    out.module = Some(input.parse::<LitStr>()?.value());
                }
                "name" => {
                    input.parse::<Token![=]>()?;
                    out.name = Some(input.parse::<LitStr>()?.value());
                }
                "bound" => {
                    input.parse::<Token![=]>()?;
                    let lit = input.parse::<LitStr>()?;
                    let tokens: TokenStream = lit.value().parse().map_err(|e| {
                        syn::Error::new(lit.span(), format!("invalid `bound` tokens: {e}"))
                    })?;
                    out.bound = Some(tokens);
                }
                "untyped" => out.untyped = true,
                "sig" => {
                    let content;
                    parenthesized!(content in input);
                    out.overloads.push(parse_overload(&content)?);
                }
                "product" => {
                    let content;
                    parenthesized!(content in input);
                    out.overloads.extend(parse_product(&content)?);
                }
                other => {
                    return Err(syn::Error::new(
                        key.span(),
                        format!("unknown #[native_fn] argument `{other}`"),
                    ));
                }
            }
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }
        Ok(out)
    }
}

/// Parses `<ty>,* -> <ty>` (the body of a `sig(...)`).
fn parse_overload(input: ParseStream) -> syn::Result<Overload> {
    let mut params = Vec::new();
    loop {
        if input.peek(Token![->]) {
            break;
        }
        params.push(parse_type(input)?);
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        } else {
            break;
        }
    }
    input.parse::<Token![->]>()?;
    let ret = parse_type(input)?;
    Ok(Overload { params, ret })
}

/// Parses `[<ty>,*], [<ty>,*], ... -> <ty>` and expands the cartesian product
/// of the slots into one overload per combination (all sharing the return).
fn parse_product(input: ParseStream) -> syn::Result<Vec<Overload>> {
    let mut slots: Vec<Vec<TokenStream>> = Vec::new();
    loop {
        if input.peek(Token![->]) {
            break;
        }
        let content;
        bracketed!(content in input);
        let mut opts = Vec::new();
        loop {
            opts.push(parse_type(&content)?);
            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            } else {
                break;
            }
        }
        slots.push(opts);
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        } else {
            break;
        }
    }
    input.parse::<Token![->]>()?;
    let ret = parse_type(input)?;

    // cartesian product of the slot option lists
    let mut combos: Vec<Vec<TokenStream>> = vec![vec![]];
    for slot in &slots {
        let mut next = Vec::new();
        for prefix in &combos {
            for opt in slot {
                let mut c = prefix.clone();
                c.push(opt.clone());
                next.push(c);
            }
        }
        combos = next;
    }
    Ok(combos
        .into_iter()
        .map(|params| Overload {
            params,
            ret: ret.clone(),
        })
        .collect())
}

/// Parses one type in the signature DSL into a token stream that constructs the
/// corresponding `rl_ast::statements::TypeAnnotation`.
fn parse_type(input: ParseStream) -> syn::Result<TokenStream> {
    let ta = quote!(::rl_ast::statements::TypeAnnotation);

    // `_` -> an anonymous generic (matches any type, as the hand-written
    // signatures use `Generic("_")`).
    if input.peek(Token![_]) {
        input.parse::<Token![_]>()?;
        return Ok(quote!(#ta::Generic("_".to_string())));
    }

    // `fn` is a Rust keyword, so it can't be parsed as an `Ident`.
    if input.peek(Token![fn]) {
        input.parse::<Token![fn]>()?;
        return Ok(quote!(#ta::Fn));
    }

    let ident: Ident = input.parse()?;
    let name = ident.to_string();
    let scalar = |t: TokenStream| Ok(t);
    match name.as_str() {
        "int" => scalar(quote!(#ta::Int)),
        "uint" => scalar(quote!(#ta::UInt)),
        "sint" => scalar(quote!(#ta::SInt)),
        "suint" => scalar(quote!(#ta::SUInt)),
        "float" => scalar(quote!(#ta::Float)),
        "sfloat" => scalar(quote!(#ta::SFloat)),
        "bool" => scalar(quote!(#ta::Bool)),
        "string" => scalar(quote!(#ta::String)),
        "char" => scalar(quote!(#ta::Char)),
        "byte" => scalar(quote!(#ta::Byte)),
        "sbyte" => scalar(quote!(#ta::SByte)),
        "bbyte" => scalar(quote!(#ta::BByte)),
        "bsbyte" => scalar(quote!(#ta::BSByte)),
        "null" => scalar(quote!(#ta::Null)),
        "any" => scalar(quote!(#ta::Infer)),
        "fn" => scalar(quote!(#ta::Fn)),
        // single-letter generics (T, U, ...)
        _ if name.len() == 1 && name.chars().next().unwrap().is_ascii_uppercase() => {
            scalar(quote!(#ta::Generic(#name.to_string())))
        }
        "result" => {
            let content;
            bracketed!(content in input);
            let inner = parse_type(&content)?;
            scalar(quote!(#ta::Result(Box::new(#inner))))
        }
        "array" => {
            let content;
            bracketed!(content in input);
            let inner = parse_type(&content)?;
            scalar(quote!(#ta::Array(Box::new(#inner))))
        }
        "set" => {
            let content;
            bracketed!(content in input);
            let inner = parse_type(&content)?;
            scalar(quote!(#ta::Set(Box::new(#inner))))
        }
        "map" => {
            let content;
            bracketed!(content in input);
            let k = parse_type(&content)?;
            content.parse::<Token![,]>()?;
            let v = parse_type(&content)?;
            scalar(quote!(#ta::Map(Box::new(#k), Box::new(#v))))
        }
        "tuple" => {
            let content;
            bracketed!(content in input);
            let mut elems = Vec::new();
            loop {
                elems.push(parse_type(&content)?);
                if content.peek(Token![,]) {
                    content.parse::<Token![,]>()?;
                } else {
                    break;
                }
            }
            scalar(quote!(#ta::Tuple(::std::rc::Rc::new(vec![#(#elems),*]))))
        }
        "handle" => {
            let content;
            parenthesized!(content in input);
            let kind: Ident = content.parse()?;
            scalar(quote!(#ta::Handle(::rl_ast::statements::HandleKind::#kind)))
        }
        "callback" => {
            let content;
            parenthesized!(content in input);
            let mut params = Vec::new();
            loop {
                if content.peek(Token![->]) {
                    break;
                }
                params.push(parse_type(&content)?);
                if content.peek(Token![,]) {
                    content.parse::<Token![,]>()?;
                } else {
                    break;
                }
            }
            content.parse::<Token![->]>()?;
            let ret = parse_type(&content)?;
            scalar(quote!(#ta::Callback(vec![#(#params),*], Box::new(#ret))))
        }
        other => Err(syn::Error::new(
            ident.span(),
            format!("unknown type `{other}` in #[native_fn] signature"),
        )),
    }
}
