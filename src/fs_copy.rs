use std::fs;
use std::io;
use std::io::{Error, ErrorKind};
use std::path::Path;

pub fn do_backup() {
    // prendere il path src e dst dal file che li contiene
    let dst = Path::new("/Users/Alessandro/Desktop/backup_prova");
    let src = Path::new("/Users/Alessandro/Desktop/Laboratori_RUST");

    copy_dir_recursive(src, dst, true);
}
fn copy_dir_recursive(src: &Path, dst: &Path, first: bool) -> io::Result<()> {

    if !dst.exists() && first{
        return io::Result::Err(Error::new(ErrorKind::NotFound, "Path inesistente"));
    }

    if !dst.exists() {
        fs::create_dir(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dst.join(entry.file_name());

        if path.is_dir() {
            copy_dir_recursive(&path, &dest_path, false)?;
        } else {
            fs::copy(&path, &dest_path)?;
        }
    }

    Ok(())
}
