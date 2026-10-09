use std::ffi::OsStr;
use std::path::Path;
use walkdir::WalkDir;

pub async fn scan_library(pathStr: String)  {
    println!("Looking for library at {}", pathStr);
    // Check if path is valid:
    let path = Path::new(pathStr.trim());

    if path.is_dir(){
        for entry in WalkDir::new(path) {
            let entry = match entry {
                Ok(e) => e,
                Err(e) => continue,
            };

            if let Some(ext) = entry.path().extension() {

                if ext == "pdf" {
                    println!("PDF! {}", entry.path().display());
                    continue;
                }

                // Add more extensions here as we go.


            } else {
                println!("File with no extension detected: {}", entry.path().display());

            }
        }

    } else {
        println!("No path found");
        eprintln!("Failed to start PaperFennec! - Could not find library at {}", pathStr);
        std::process::exit(1);
    }


}