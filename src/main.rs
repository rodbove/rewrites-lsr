use std::fs;
use colored::*;
use clap::Parser;

#[derive(Parser)]
#[command(name = "lsr", about = "ls replacement in Rust")]
struct Args {
    #[arg(default_value = ".")]
    path: String,

    #[arg(short, long)]
    all: bool,
}

fn main() {
    let args = Args::parse();

    let paths = fs::read_dir(&args.path).unwrap();

    let mut entries: Vec<_> = paths
        .map(|entry| entry.unwrap())
        .collect();

    entries.sort_by_key(|entry| entry.file_name());

    for entry in entries {
        let metadata = entry.metadata().unwrap();
        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        if !args.all && name_str.starts_with('.') {
            continue;
        }

        let size = metadata.len();
        let size_str = format_size(size);

        if metadata.is_dir() {
            println!("{:>8}  {}", size_str, format!("{}/", name_str.blue()));
        } else {
            println!("{:>8}  {}", size_str, name_str);
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