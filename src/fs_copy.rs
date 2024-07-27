use std::fmt::Debug;
use std::fs;
use std::io;
use std::path::Path;
use chrono::Local;
use crate::config::Config;

pub fn do_backup(){

    let mut config = Config::default();
    config.load();

    let src : &Path = Path::new(config.source.as_str());
    let dst : &Path = Path::new(config.destination.as_str());
    let target_ext = config.extension.as_str();

    if src.exists() && dst.exists() {
        let time = Local::now().format("%Y-%m-%d_%H:%M:%S").to_string();
        let new_name = format!("{}_{}", src.file_name().unwrap().to_str().unwrap(), time);
        let dest_new_path = dst.join(new_name);
        fs::create_dir(dest_new_path.clone()).unwrap();
        match copy_dir_recursive(src, dest_new_path.as_path(), target_ext) {
            Ok(_) => {}
            Err(_) => {eprintln!("Error copying files.")}
        };
    }
    else { eprintln!("Directory does not exist!") }

}
fn copy_dir_recursive(src: &Path, dst: &Path, target_ext: &str)-> io::Result<()>{

    if !dst.exists()  {
        fs::create_dir(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dst.join(entry.file_name());

        if path.is_dir() {
            copy_dir_recursive(&path, &dest_path, target_ext)?;
        } else {
            if let Some(ext) = path.extension() {
                if target_ext.len() == 0 || ext.to_str().unwrap() == target_ext {
                    fs::copy(&path, &dest_path)?;
                }
            }
        }
    }

    Ok(())
}
