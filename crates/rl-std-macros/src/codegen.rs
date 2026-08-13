//! Code generation for `#[native_fn]`: turns the parsed attribute and the
//! annotated function into the wrapper / signature / handle trio.

use crate::attr::NativeFnAttr;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{FnArg, ItemFn, PatType, Type, parse_quote, spanned::Spanned};

/// How a single Rust parameter maps onto the calling convention.
enum Param {
    /// The `&mut R::Cx` runtime context (forwarded, not an rl argument).
    Ctx,
    /// The `R::Span` call span.
    Span,
    /// A `Vec<R::Value>` variadic argument (the whole argument vector).
    VarArgs,
    /// A raw `R::Value` argument (identity extraction).
    Raw,
    /// A typed argument extracted via `FromValueR`.
    Typed(Box<Type>),
}

/// How the Rust return type maps onto the produced value.
enum Ret {
    /// Plain `T` (including `()`); converted via `IntoValueR`.
    Concrete(Box<Type>),
    /// `R::Value`; returned as-is.
    Raw,
    /// `Result<T, Error>`; the error propagates, the ok value is `T`.
    Fallible(Box<Type>),
    /// `Result<T, String>`; becomes a language `result[T]` (Ok/Err value).
    LangResult(Box<Type>),
}

pub fn expand(attr: NativeFnAttr, func: ItemFn) -> syn::Result<TokenStream> {
    let module = attr.module.clone().ok_or_else(|| {
        syn::Error::new(
            func.sig.ident.span(),
            "#[native_fn] requires `module = \"...\"`",
        )
    })?;
    let _ = module; // module name is used by the aggregating builder, not here

    let fn_ident = func.sig.ident.clone();
    let rl_name = attr.name.clone().unwrap_or_else(|| fn_ident.to_string());
    let has_r_generic = func.sig.generics.type_params().any(|tp| tp.ident == "R");

    // classify parameters
    let mut params = Vec::new();
    for (i, arg) in func.sig.inputs.iter().enumerate() {
        let FnArg::Typed(PatType { ty, .. }) = arg else {
            return Err(syn::Error::new(
                arg.span(),
                "#[native_fn] does not support `self`",
            ));
        };
        params.push(classify_param(ty, i == 0)?);
    }
    let ret = classify_ret(&func.sig.output);

    // ---- arity + whether we need explicit signatures ----------------------
    let has_varargs = params.iter().any(|p| matches!(p, Param::VarArgs));
    let value_count = params
        .iter()
        .filter(|p| matches!(p, Param::Raw | Param::Typed(_)))
        .count();
    let has_raw =
        params.iter().any(|p| matches!(p, Param::Raw)) || has_varargs || matches!(ret, Ret::Raw);

    // ---- build the wrapper body -------------------------------------------
    let rt = quote!(::rl_std_core::Runtime);
    let arity_check = if has_varargs {
        quote!()
    } else {
        quote! {
            if args.len() != #value_count {
                return Err(<R as #rt>::error(
                    &*cx,
                    format!("{}: expected {} argument(s), got {}", #rl_name, #value_count, args.len()),
                    span,
                ));
            }
        }
    };
    let iter_decl = if value_count > 0 && !has_varargs {
        quote!(let mut __it = args.into_iter();)
    } else {
        quote!()
    };

    let mut stmts = Vec::new();
    let mut forward = Vec::new();
    let mut value_idx = 0usize;
    for p in &params {
        match p {
            Param::Ctx => forward.push(quote!(cx)),
            Param::Span => forward.push(quote!(span)),
            Param::VarArgs => forward.push(quote!(args)),
            Param::Raw => {
                let id = format_ident!("__arg{}", value_idx);
                value_idx += 1;
                stmts.push(quote!(let #id = __it.next().unwrap();));
                forward.push(quote!(#id));
            }
            Param::Typed(ty) => {
                let id = format_ident!("__arg{}", value_idx);
                value_idx += 1;
                let expected = rl_type_name(ty);
                stmts.push(quote! {
                    let #id = match <#ty as ::rl_std_core::FromValueR<R>>::from_value(__it.next().unwrap()) {
                        Ok(v) => v,
                        Err(__bad) => {
                            return Err(<R as #rt>::error(
                                &*cx,
                                format!("{}: expected {}, got {}", #rl_name, #expected, <R as #rt>::type_name(&__bad)),
                                span,
                            ));
                        }
                    };
                });
                forward.push(quote!(#id));
            }
        }
    }

    let turbofish = if has_r_generic {
        quote!(::<R>)
    } else {
        quote!()
    };
    let call = quote!(super::#fn_ident #turbofish (#(#forward),*));

    let ret_expr = match &ret {
        Ret::Raw => quote!(Ok(#call)),
        Ret::Concrete(ty) => {
            quote!(Ok(<#ty as ::rl_std_core::IntoValueR<R>>::into_value(#call)))
        }
        Ret::Fallible(inner) => {
            if is_r_value(inner) {
                // the body already returns `Result<R::Value, Error>`; wrapping
                // it in `Ok(.. ?)` would be a needless round-trip (clippy).
                quote!(#call)
            } else {
                quote!(Ok(<#inner as ::rl_std_core::IntoValueR<R>>::into_value(#call?)))
            }
        }
        Ret::LangResult(inner) => {
            let ok_val = if is_r_value(inner) {
                quote!(__v)
            } else {
                quote!(<#inner as ::rl_std_core::IntoValueR<R>>::into_value(__v))
            };
            quote! {
                match #call {
                    Ok(__v) => Ok(<R as #rt>::ok(#ok_val)),
                    Err(__e) => Ok(<R as #rt>::err(<R as #rt>::from_string(__e))),
                }
            }
        }
    };

    // The `R` bound for the generated wrapper/handle: `Runtime`, plus any extra
    // per-module bound (e.g. a handle-store trait) supplied via `bound = "..."`.
    let r_bound = match &attr.bound {
        Some(b) => quote!(::rl_std_core::Runtime + #b),
        None => quote!(::rl_std_core::Runtime),
    };

    let wrapper = quote! {
        #[allow(unused_variables, unused_mut, clippy::let_unit_value)]
        pub fn wrapper<R: #r_bound>(
            cx: &mut <R as ::rl_std_core::Runtime>::Cx,
            args: ::std::vec::Vec<<R as ::rl_std_core::Runtime>::Value>,
            span: <R as ::rl_std_core::Runtime>::Span,
        ) -> ::std::result::Result<<R as ::rl_std_core::Runtime>::Value, ::rl_utils::errors::Error> {
            #arity_check
            #iter_decl
            #(#stmts)*
            #ret_expr
        }
    };

    // ---- signature() ------------------------------------------------------
    let ta = quote!(::rl_ast::statements::TypeAnnotation);
    let signature = if attr.untyped {
        quote!(::rl_std_core::StdFn::untyped(#rl_name))
    } else if !attr.overloads.is_empty() {
        let rows = attr.overloads.iter().map(|o| {
            let ps = &o.params;
            let r = &o.ret;
            quote!((#ta::Tuple(::std::rc::Rc::new(vec![#(#ps),*])), #r))
        });
        quote!(::rl_std_core::StdFn::typed(#rl_name, vec![#(#rows),*]))
    } else {
        // auto-derive: only valid when there are no raw params/return
        if has_raw {
            return Err(syn::Error::new(
                func.sig.ident.span(),
                "#[native_fn] cannot derive a signature for a function using `R::Value`; \
                 provide an explicit `sig(...)`/`product(...)` or `untyped`",
            ));
        }
        let param_anns = params.iter().filter_map(|p| match p {
            Param::Typed(ty) => Some(quote!(<#ty as ::rl_std_core::ValueType>::type_annotation())),
            _ => None,
        });
        let ret_ann = match &ret {
            Ret::Concrete(ty) => quote!(<#ty as ::rl_std_core::ValueType>::type_annotation()),
            Ret::Fallible(inner) => quote!(<#inner as ::rl_std_core::ValueType>::type_annotation()),
            Ret::LangResult(inner) => {
                quote!(#ta::Result(Box::new(<#inner as ::rl_std_core::ValueType>::type_annotation())))
            }
            Ret::Raw => unreachable!("has_raw guard above"),
        };
        quote! {
            ::rl_std_core::StdFn::typed(
                #rl_name,
                vec![(#ta::Tuple(::std::rc::Rc::new(vec![#(#param_anns),*])), #ret_ann)],
            )
        }
    };

    // ---- handle() ---------------------------------------------------------
    let arity_expr = if has_varargs {
        quote!(::rl_std_core::Arity::Variadic)
    } else {
        quote!(::rl_std_core::Arity::Fixed(#value_count))
    };

    // `signature()`/`NAME` are pure `TypeAnnotation` data and stay available in
    // a signatures-only build (the checker/LSP). The body, wrapper, and handle
    // need the runtime and any OS-facing deps, so they live behind the `impls`
    // feature - this is what lets the checker read every module's signatures
    // without compiling eframe/rodio/libffi.
    Ok(quote! {
        #[cfg(feature = "impls")]
        #func

        #[doc(hidden)]
        pub mod #fn_ident {
            /// The rl-level name of this function.
            pub const NAME: &str = #rl_name;

            pub fn signature() -> ::rl_std_core::StdFn {
                #signature
            }

            #[cfg(feature = "impls")]
            #[allow(unused_imports)]
            use super::*;

            #[cfg(feature = "impls")]
            #wrapper

            #[cfg(feature = "impls")]
            pub fn handle<R: #r_bound>() -> ::rl_std_core::NativeHandle<R> {
                ::rl_std_core::NativeHandle {
                    name: #rl_name,
                    arity: #arity_expr,
                    thunk: wrapper::<R>,
                    sig: signature,
                }
            }
        }
    })
}

fn classify_param(ty: &Type, is_first: bool) -> syn::Result<Param> {
    if is_first && is_mut_ref(ty) {
        return Ok(Param::Ctx);
    }
    if is_r_span(ty) {
        return Ok(Param::Span);
    }
    if is_vec_r_value(ty) {
        return Ok(Param::VarArgs);
    }
    if is_r_value(ty) {
        return Ok(Param::Raw);
    }
    Ok(Param::Typed(Box::new(ty.clone())))
}

fn classify_ret(output: &syn::ReturnType) -> Ret {
    let ty = match output {
        syn::ReturnType::Default => return Ret::Concrete(Box::new(parse_quote!(()))),
        syn::ReturnType::Type(_, ty) => ty.as_ref(),
    };
    if is_r_value(ty) {
        return Ret::Raw;
    }
    if let Some((ok, err)) = as_result(ty) {
        if is_error_type(err) {
            return Ret::Fallible(Box::new(ok.clone()));
        }
        return Ret::LangResult(Box::new(ok.clone()));
    }
    Ret::Concrete(Box::new(ty.clone()))
}

// ---- structural type predicates -------------------------------------------

fn path_idents(ty: &Type) -> Option<Vec<String>> {
    match ty {
        Type::Path(tp) if tp.qself.is_none() => Some(
            tp.path
                .segments
                .iter()
                .map(|s| s.ident.to_string())
                .collect(),
        ),
        _ => None,
    }
}

fn is_r_value(ty: &Type) -> bool {
    path_idents(ty).as_deref() == Some(&["R".to_string(), "Value".to_string()])
}

fn is_r_span(ty: &Type) -> bool {
    path_idents(ty).as_deref() == Some(&["R".to_string(), "Span".to_string()])
}

fn is_mut_ref(ty: &Type) -> bool {
    matches!(ty, Type::Reference(r) if r.mutability.is_some())
}

/// The single type argument of a one-parameter generic (`Vec<X>` -> `X`).
fn single_generic_arg<'a>(ty: &'a Type, name: &str) -> Option<&'a Type> {
    let Type::Path(tp) = ty else { return None };
    let seg = tp.path.segments.last()?;
    if seg.ident != name {
        return None;
    }
    let syn::PathArguments::AngleBracketed(args) = &seg.arguments else {
        return None;
    };
    match args.args.first()? {
        syn::GenericArgument::Type(t) => Some(t),
        _ => None,
    }
}

fn is_vec_r_value(ty: &Type) -> bool {
    single_generic_arg(ty, "Vec").is_some_and(is_r_value)
}

fn as_result(ty: &Type) -> Option<(&Type, &Type)> {
    let Type::Path(tp) = ty else { return None };
    let seg = tp.path.segments.last()?;
    if seg.ident != "Result" {
        return None;
    }
    let syn::PathArguments::AngleBracketed(args) = &seg.arguments else {
        return None;
    };
    let mut types = args.args.iter().filter_map(|a| match a {
        syn::GenericArgument::Type(t) => Some(t),
        _ => None,
    });
    let ok = types.next()?;
    let err = types.next()?;
    Some((ok, err))
}

fn is_error_type(ty: &Type) -> bool {
    match path_idents(ty) {
        Some(segs) => segs.last().map(String::as_str) == Some("Error"),
        None => false,
    }
}

/// Best-effort rl type name for argument mismatch messages.
fn rl_type_name(ty: &Type) -> String {
    if single_generic_arg(ty, "Vec").is_some() {
        return "array".to_string();
    }
    let last = path_idents(ty).and_then(|s| s.last().cloned());
    match last.as_deref() {
        Some("String") => "string",
        Some("i64") => "int",
        Some("u64") => "uint",
        Some("i32") => "small int",
        Some("u32") => "small uint",
        Some("i16") => "big sbyte",
        Some("u16") => "big byte",
        Some("i8") => "sbyte",
        Some("u8") => "byte",
        Some("f64") => "float",
        Some("f32") => "small float",
        Some("bool") => "bool",
        Some("char") => "char",
        _ => "the expected type",
    }
    .to_string()
}
