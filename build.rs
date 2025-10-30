use std::{env, path::Path, process::Command};

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir);

    let manifest_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let manifest_path = Path::new(&manifest_dir);

    Command::new("cp")
        .arg("-R")
        .arg(manifest_path.join("static"))
        .arg(out_path)
        .output()
        .expect("failed to copy /static to $OUT_DIR");
}
