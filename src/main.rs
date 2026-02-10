use std::fs;
use colored::*;

fn main() {
    let paths = fs::read_dir(".").unwrap();

    let mut entries: Vec<_> = paths
        .map(|entry| entry.unwrap())
        .collect();

    entries.sort_by_key(|entry| entry.file_name());

    for entry in entries {
        let metadata = entry.metadata().unwrap();
        let name = entry.file_name();

        if metadata.is_dir() {
            println!("{}/", name.to_string_lossy().blue());
        } else {
            println!("{}", name.to_string_lossy());
        }
    }

}
