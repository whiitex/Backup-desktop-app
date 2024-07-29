use std::fs;
use std::path::Path;

fn main() {


    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("../../../assets");
    fs::create_dir_all(&dest_path).unwrap();
    fs::copy("assets/logo.png", dest_path.join("logo.png")).unwrap();
    fs::copy("assets/rectangle_animation.gif", dest_path.join("rectangle_animation.gif")).unwrap();
    fs::copy("assets/line_animation.gif", dest_path.join("line_animation.gif")).unwrap();
    fs::copy("assets/beep.wav", dest_path.join("beep.wav")).unwrap();

    embed_resource::compile("app_icon.rc", embed_resource::NONE);
}