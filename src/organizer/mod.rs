pub mod file_types;
pub mod mover;

use std::fs;
use std::path::Path;

use crate::organizer::file_types::get_folder_for_extension;
use crate::organizer::mover::move_file;

pub fn organize_folder(folder_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let entries = fs::read_dir(folder_path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let target_folder = if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                get_folder_for_extension(ext).unwrap_or("Other")
            } else {
                "Other"
            };

            move_file(&path, folder_path, target_folder)?;
        }
    }

    Ok(())
}
