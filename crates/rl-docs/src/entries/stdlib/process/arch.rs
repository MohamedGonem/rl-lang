use crate::entry::FnEntry;

pub static ARCH: FnEntry = FnEntry {
    signature: "arch()",
    description: "returns the CPU architecture (x86_64, aarch64, etc)",
    example: r#"get std::process::arch

dec string a = arch()?"#,
    expected_output: None,
    returns: "string",
    errors: None,
    see_also: &["os_name", "num_cpus"],
    since: Some("v2.1.0"),
};
