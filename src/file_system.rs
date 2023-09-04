use std::{fs, io, path::Path};

pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    println!("->> Copying dir: {:?} to {:?}", src.as_ref(), dst.as_ref());
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let dst_path = dst.as_ref().join(entry.file_name());
        if ty.is_dir() {
            copy_dir_all(entry.path(), &dst_path)?;
        } else {
            println!("->> Copying file: {:?} to {:?}", entry.path(), dst_path);
            fs::copy(entry.path(), &dst_path)?;
        }
    }
    Ok(())
}
