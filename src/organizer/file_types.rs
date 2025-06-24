use std::collections::HashMap;

pub fn get_folder_for_extension(extension: &str) -> Option<&'static str> {
    let mapping = get_extension_mapping();
    mapping.get(extension).copied()
}

fn get_extension_mapping() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("jpg", "Images"),
        ("jpeg", "Images"),
        ("png", "Images"),
        ("gif", "Images"),
        ("pdf", "Documents"),
        ("txt", "TextFiles"),
        ("mp4", "Videos"),
    ])
}
