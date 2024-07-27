use std::fs;
use std::io;
use std::io::{Error, ErrorKind};
use std::path::Path;
use serde_json::ser::CompactFormatter;
use crate::config::Config;

pub fn do_backup(){

    let mut config = Config::default();
    config.load();

    let src : &Path = Path::new(config.source.as_str());
    let dst : &Path = Path::new(config.destination.as_str());
    let target_ext = config.extension.as_str();

    if src.exists() && dst.exists() {
        match copy_dir_recursive(src, dst,target_ext) {
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
