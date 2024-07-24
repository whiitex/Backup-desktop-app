use std::fs;
use std::path::Path;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("../../../assets");
    fs::create_dir_all(&dest_path).unwrap();
    fs::copy("assets/logo.png", dest_path.join("logo.png")).unwrap();
}