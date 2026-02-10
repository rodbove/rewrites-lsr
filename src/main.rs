use std::fs;

fn main() {
    // read current directory
    let paths = fs::read_dir(".").unwrap();

    for path in paths {
        let entry = path.unwrap();
        println!("{}", entry.file_name().to_string_lossy());
    }
}
