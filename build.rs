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
    fs::copy("assets/backup_done.mp3", dest_path.join("backup_done.mp3")).unwrap();
    fs::copy("assets/backup_draw.mp3", dest_path.join("backup_draw.mp3")).unwrap();
    fs::copy("assets/backup_failed.mp3", dest_path.join("backup_failed.mp3")).unwrap();
    fs::copy("assets/backup_started.mp3", dest_path.join("backup_started.mp3")).unwrap();

    embed_resource::compile("app_icon.rc", embed_resource::NONE);
}