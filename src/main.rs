use tidy::organizer::organize_folder;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    let folder = args.get(1).map(|s| s.as_str()).unwrap_or(".");

    if let Err(e) = organize_folder(folder) {
        eprintln!("Error: {}", e);
    }
}
