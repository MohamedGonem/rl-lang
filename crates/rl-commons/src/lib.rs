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
/// All 20 migrated modules' signatures come straight from `rl-std`'s
/// `#[native_fn]` annotations (so they can never drift from the
/// implementations, and without pulling in any OS-facing deps). Only the
/// runtime-specific `rl` module - which stays legacy because unifying it would
/// create a dependency cycle with `rl-checker` - keeps a hand-written signature
/// here.
pub fn stdlib_names() -> ModuleNames {
    rl_std::signatures().with_module(stdlib_signatures::rl::module())
}
