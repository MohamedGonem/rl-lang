use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::process::Command;

use crate::{
    Vm,
    stdlib::{
        c::{
            CHandle,
            common::{cache_dir, insert_handle},
        },
        common::extract_string,
        macros::{verr, vi, vok, vs},
    },
    values::VmValue,
};

#[cfg(target_os = "windows")]
const SHARED_LIB_EXT: &str = "dll";
#[cfg(target_os = "macos")]
const SHARED_LIB_EXT: &str = "dylib";
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
const SHARED_LIB_EXT: &str = "so";

/// Builds the compiler invocation for turning `src` into `out` on this platform.
/// macOS wants `-dynamiclib`; everything else (incl. mingw-w64 `cc`/`gcc` on
/// Windows) accepts `-shared -fPIC`. Native MSVC (`cl.exe`) is not supported.
fn compiler_command(compiler: &str, src: &PathBuf, out: &PathBuf) -> Command {
    let mut cmd = Command::new(compiler);
    #[cfg(target_os = "macos")]
    {
        cmd.args(["-dynamiclib", "-O2", "-o"]).arg(out).arg(src);
    }
    #[cfg(not(target_os = "macos"))]
    {
        cmd.args(["-shared", "-fPIC", "-O2", "-o"])
            .arg(out)
            .arg(src);
    }
    cmd
}

fn find_compiler() -> String {
    std::env::var("CC").unwrap_or_else(|_| "cc".to_string())
}

pub fn std_compile(vm: &mut Vm, source: VmValue) -> VmValue {
    let source = match extract_string(source, "compile") {
        Ok(s) => s,
        Err(e) => return verr!(vs!(format!("compile: {}", e))),
    };

    let mut hasher = DefaultHasher::new();
    source.hash(&mut hasher);
    let hash = hasher.finish();

    let dir = cache_dir();
    if let Err(e) = std::fs::create_dir_all(&dir) {
        return verr!(vs!(format!(
            "compile: failed to create cache dir {}: {}",
            dir.display(),
            e
        )));
    }

    let src_path = dir.join(format!("{:016x}.c", hash));
    let out_path = dir.join(format!("{:016x}.{}", hash, SHARED_LIB_EXT));

    // Cache hit: skip both writing the source and recompiling.
    if !out_path.exists() {
        if let Err(e) = std::fs::write(&src_path, &source) {
            return verr!(vs!(format!(
                "compile: failed to write {}: {}",
                src_path.display(),
                e
            )));
        }

        let compiler = find_compiler();
        let output = compiler_command(&compiler, &src_path, &out_path).output();

        match output {
            Ok(o) if o.status.success() => {}
            Ok(o) => {
                let stderr = String::from_utf8_lossy(&o.stderr);
                return verr!(vs!(format!(
                    "compile: `{}` failed:\n{}",
                    compiler,
                    stderr.trim_end()
                )));
            }
            Err(e) => {
                return verr!(vs!(format!(
                    "compile: couldn't run `{}` (set $CC to point at a C compiler if it's not on $PATH): {}",
                    compiler, e
                )));
            }
        }
    }

    // SAFETY: loading the shared library we just compiled from a fixed cache
    // path. The library's static initializers (if any) run here.
    let lib = match unsafe { libloading::Library::new(&out_path) } {
        Ok(l) => l,
        Err(e) => {
            return verr!(vs!(format!(
                "compile: failed to load compiled library {}: {}",
                out_path.display(),
                e
            )));
        }
    };

    let id = insert_handle(vm, CHandle::Library(lib));
    vok!(vi!(id))
}
