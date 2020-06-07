fn main() {
    use std::env::var;

    let crate_dir =
        var("CARGO_MANIFEST_DIR").expect("Unable to get environment variable $CARGO_MANIFEST_DIR");

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .generate()
        .expect("Unable to generate chua4c.h")
        .write_to_file("target/include/chua4c.h");
}
