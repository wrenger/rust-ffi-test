use std::env;
use std::path::PathBuf;

const BINDINGS_HEADERS: [&str; 1] = [
    "include/api.h",
];
const INCLUDE_DIRS: [&str; 1] = [
    "include/",
];
const BINDINGS_OUT: &str = "bindings.rs";

fn main() {
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = PathBuf::from(".");

    let include_args = INCLUDE_DIRS.iter().map(|dir| {
        format!("-I{}", &[project_dir.as_str(), dir].join("/"))
    }).collect::<Vec<_>>();

    let mut builder = bindgen::builder()
        .allowlist_function("Point2_.*")
        .allowlist_type("Point2.*")
        .allowlist_var("Point2_.*")
        .clang_args(&include_args);

    for &header in &BINDINGS_HEADERS {
        builder = builder.header(header);
    }

    let bindings = builder.generate().expect("Bindings generation failed");

    bindings
        .write_to_file(out_dir.join(BINDINGS_OUT))
        .expect("Bindings saving failed");
}
