use crate::entry::FnEntry;

pub static NUM_CPUS: FnEntry = FnEntry {
    signature: "num_cpus()",
    description: "returns the number of available CPU cores",
    example: r#"get std::process::num_cpus

dec int cores = num_cpus()?"#,
    expected_output: None,
    returns: "int",
    errors: None,
    see_also: &["arch"],
    since: None,
};
