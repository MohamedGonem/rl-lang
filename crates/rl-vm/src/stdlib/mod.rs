//! The VM's standard library - built-in modules registered under `std::*`.

// `common`/`macros` now serve only the legacy `rl` and `len` functions; some of
// their helpers are unused until those migrate (`macros.rs` allows this itself).
#[allow(dead_code, unused_macros, unused_imports)]
pub mod common;
mod len;
mod macros;
mod rl;

use crate::native::Module;
use crate::runtime::VmRuntime;

/// Builds the compiler-facing native module tree: an unnamed root holding
/// a `std` submodule, mirroring the shared `rl-std` module shape so
/// `std::io::println` resolves correctly.
pub fn root() -> Module {
    Module::new("root").with_module(
        Module::new("std")
            .with_module(Module::from_std("io", rl_std::io::handles::<VmRuntime>()))
            .with_module(Module::from_std(
                "collections",
                rl_std::collections::handles::<VmRuntime>(),
            ))
            .with_module(
                Module::from_std("array", rl_std::array::handles::<VmRuntime>())
                    .with_function("len", len::std_len),
            )
            .with_function("len", len::std_len)
            .with_module(Module::from_std("c", rl_std::c::handles::<VmRuntime>()))
            .with_module(Module::from_std(
                "audio",
                rl_std::audio::handles::<VmRuntime>(),
            ))
            .with_module(Module::from_std("gui", rl_std::gui::handles::<VmRuntime>()))
            .with_module(Module::from_std(
                "bitwise",
                rl_std::bitwise::handles::<VmRuntime>(),
            ))
            .with_module(Module::from_std(
                "debug",
                rl_std::debug::handles::<VmRuntime>(),
            ))
            .with_module(Module::from_std("fs", rl_std::fs::handles::<VmRuntime>()))
            .with_module(Module::from_std(
                "http",
                rl_std::http::handles::<VmRuntime>(),
            ))
            .with_module(
                Module::from_std("math", rl_std::math::handles::<VmRuntime>()).with_module(
                    Module::from_std("consts", rl_std::math::constants::handles::<VmRuntime>()),
                ),
            )
            .with_module(Module::from_std("net", rl_std::net::handles::<VmRuntime>()))
            .with_module(Module::from_std(
                "path",
                rl_std::path::handles::<VmRuntime>(),
            ))
            .with_module(Module::from_std(
                "process",
                rl_std::process::handles::<VmRuntime>(),
            ))
            .with_module(Module::from_std(
                "random",
                rl_std::random::handles::<VmRuntime>(),
            ))
            .with_module(Module::from_std(
                "res",
                rl_std::result::handles::<VmRuntime>(),
            ))
            .with_module(rl::module())
            .with_module(Module::from_std(
                "str",
                rl_std::string::handles::<VmRuntime>(),
            ))
            .with_module(Module::from_std(
                "term",
                rl_std::terminal::handles::<VmRuntime>(),
            ))
            .with_module(Module::from_std(
                "time",
                rl_std::time::handles::<VmRuntime>(),
            ))
            .with_module(Module::from_std(
                "types",
                rl_std::types::handles::<VmRuntime>(),
            )),
    )
}
