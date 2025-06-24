use std::fs;
use std::path::{Path, PathBuf};

pub fn move_file(
    file_path: &Path,
    base_folder: &str,
    target_folder: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let target_dir = Path::new(base_folder).join(target_folder);

    if !target_dir.exists() {
        fs::create_dir_all(&target_dir)?;
    }

    let file_name = file_path.file_name().unwrap();
    let new_location = target_dir.join(file_name);

    fs::rename(file_path, new_location)?;

    Ok(())
}
