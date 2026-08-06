pub mod keywords;
pub mod stdlib_signatures;

// `StdFn`/`ModuleNames` now live in `rl-std-core` so the single stdlib source
// (`rl-std`) can emit signatures next to the implementations. They are
// re-exported here so existing consumers (`rl-checker`, `rl-lsp`) keep
// compiling unchanged while the stdlib is migrated. The hand-written
// `stdlib_signatures` tables below are retired once every module has moved to
// `rl-std`'s `#[native_fn]` signatures.
pub use rl_std_core::{ModuleNames, StdFn};

/// The full `std::*` signature tree consumed by the checker and LSP.
///
/// The 14 pure-`std` modules' signatures come straight from `rl-std`'s
/// `#[native_fn]` annotations (so they can never drift from the
/// implementations). The six OS-facing modules (`audio`, `c`, `gui`, `http`,
/// `process`, `terminal`) and the runtime-specific `rl` module keep their
/// hand-written signatures here for now, since their `rl-std` sources are gated
/// behind the `impls` feature (see `rl-std`'s crate docs).
pub fn stdlib_names() -> ModuleNames {
    rl_std::signatures()
        .with_module(stdlib_signatures::audio::module())
        .with_module(stdlib_signatures::c::module())
        .with_module(stdlib_signatures::gui::module())
        .with_module(stdlib_signatures::http::module())
        .with_module(stdlib_signatures::process::module())
        .with_module(stdlib_signatures::terminal::module())
        .with_module(stdlib_signatures::rl::module())
}
