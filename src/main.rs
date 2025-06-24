use clap::Parser;
use tidy::organizer::organize_folder;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    path: Option<String>,
}

fn main() {
    let args = Args::parse();
    let folder_path = args.path.unwrap_or_else(|| ".".to_string());

    if let Err(e) = organize_folder(&folder_path) {
        eprintln!("Error: {}", e);
    }
}
