use crate::entry::FnEntry;

pub static CLEAR_CACHE: FnEntry = FnEntry {
    signature: "clear_cache()",
    description: "deletes `compile`'s on-disk cache directory (built shared libraries and their `.c` sources, keyed by content hash) - the next `compile` call for any source recompiles from scratch and repopulates it. Safe to call even while handles from previously-compiled libraries are still open, on POSIX (a `dlopen`'d library stays mapped by inode even after its file is unlinked). On Windows this is less certain - deleting a file a running process still has mapped can fail there, so `close` the relevant handles first if `clear_cache` errors. A missing cache directory (nothing ever compiled, or already cleared) is not an error - `clear_cache` is idempotent",
    example: r#"
get std::c::clear_cache

clear_cache()?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        "err(string) if the cache directory exists but can't be removed (e.g. a file within it is still in use on Windows)",
    ),
    see_also: &["compile"],
    since: Some("v0.4.1"),
};
