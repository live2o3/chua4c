fn main() {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("Unable to get environment variable $CARGO_MANIFEST_DIR");

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .generate()
        .expect("Unable to generate chua4c.h")
        .write_to_file("include/chua4c.h");
}
