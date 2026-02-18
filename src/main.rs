use std::fs;
use std::fs::DirEntry;
use std::process;
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

    let paths = match fs::read_dir(&args.path) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error reading '{}': {}", args.path, e);
            process::exit(1);
        }
    };

    let mut entries: Vec<DirEntry> = paths
        .filter_map(|entry| {
            match entry {
                Ok(e) => Some(e),
                Err(error) => {
                    eprintln!("Warning: {}", error);
                    None
                }
            }
        })
        .collect();

    entries.sort_by_key(|entry| entry.file_name());

    for entry in entries {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        if !args.all && name_str.starts_with('.') {
            continue;
        }

        let metadata = match entry.metadata() {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Warning: couldn't read {}: {}", name_str, e);
                continue;
            }
        };

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