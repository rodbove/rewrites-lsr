use std::fs;
use colored::*;

fn main() {
    let paths = fs::read_dir(".").unwrap();

    for path in paths {
        let entry = path.unwrap();
        let metadata = entry.metadata().unwrap();
        let name = entry.file_name();

        if metadata.is_dir() {
            println!("{}/", name.to_string_lossy().blue());
        } else {
            println!("{}", name.to_string_lossy());
        }
    }

}
