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
        let size = metadata.len();

        let size_str = format_size(size);

        if metadata.is_dir() {
            println!("{:>8}  {}", size_str, format!("{}/", name.to_string_lossy().blue()));
        } else {
            println!("{:>8}  {}", size_str, name.to_string_lossy());
        }
    }

}

fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    
    if bytes >= GB {
        format!("{:.1}G", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1}M", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1}K", bytes as f64 / KB as f64)
    } else {
        format!("{}B", bytes)
    }
}