//! Helpers for aggregating `#[native_fn]`-generated functions into a module's
//! runtime handle table and its checker signature tree.

/// Builds a module's `handles::<R>()` (runtime registration) and `signatures()`
/// (checker) from a list of `#[native_fn]`-annotated function names, plus any
/// nested sub-modules.
///
/// ```ignore
/// rl_std_core::native_module!("str";
///     funcs: [to_upper, to_lower, contains, split, join],
/// );
/// // with sub-modules:
/// rl_std_core::native_module!("math";
///     funcs: [abs, pow, sqrt],
///     mods: [constants],
/// );
/// ```
///
/// Each `func` names a `mod <func>` emitted by `#[native_fn]` (which exposes
/// `handle::<R>()` and `signature()`). Each `mod` names a child module exposing
/// its own `handles::<R>()` / `signatures()`.
#[macro_export]
macro_rules! native_module {
    // Handle-module variant: an extra per-domain bound (e.g. `NetStore`) is
    // required on `R` for `handles::<R>()`.
    (
        $name:literal;
        bound: $bound:path;
        funcs: [ $($func:ident),* $(,)? ] $(,)?
    ) => {
        /// The runtime native-function handles for this module (needs `impls`).
        #[cfg(feature = "impls")]
        pub fn handles<R: $crate::Runtime + $bound>() -> ::std::vec::Vec<$crate::NativeHandle<R>> {
            ::std::vec![ $( $func::handle::<R>() ),* ]
        }

        /// The checker signature tree for this module (always available).
        pub fn signatures() -> $crate::ModuleNames {
            #[allow(unused_mut)]
            let mut m = $crate::ModuleNames::new($name);
            $( m = m.with_typed_function($func::signature()); )*
            m
        }

        /// The function names in this module, for "did you mean" suggestions.
        pub const KEYWORDS: &[&str] = &[ $( $func::NAME ),* ];
    };
    (
        $name:literal;
        funcs: [ $($func:ident),* $(,)? ]
        $(, mods: [ $($sub:ident),* $(,)? ] )? $(,)?
    ) => {
        /// The runtime native-function handles for this module (needs `impls`).
        #[cfg(feature = "impls")]
        pub fn handles<R: $crate::Runtime>() -> ::std::vec::Vec<$crate::NativeHandle<R>> {
            ::std::vec![ $( $func::handle::<R>() ),* ]
        }

        /// The checker signature tree for this module (always available).
        pub fn signatures() -> $crate::ModuleNames {
            #[allow(unused_mut)]
            let mut m = $crate::ModuleNames::new($name);
            $( m = m.with_typed_function($func::signature()); )*
            $( $( m = m.with_module($sub::signatures()); )* )?
            m
        }

        /// The function names in this module, for "did you mean" suggestions.
        pub const KEYWORDS: &[&str] = &[ $( $func::NAME ),* ];
    };
}
