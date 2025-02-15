// funciton that can help with updating states' compoennts

use std::fs;
use std::io;
use std::path::Path;

pub fn visit_dirs(dir: &str) -> io::Result<Vec<fs::DirEntry>> {
    fs::read_dir(Path::new(dir))?.collect()
}

pub fn print_file(entry: &fs::DirEntry) {
    let file_name = entry
        .file_name()
        .into_string()
        .unwrap_or("Not unwrapped".to_string());

    let file_type = if entry.path().is_dir() {
        "dir "
    } else {
        "file"
    };
    println!("{} | {}", file_type, file_name);
}

pub fn get_file_name(entry: &fs::DirEntry) -> String {
    entry
        .file_name()
        .as_os_str()
        .to_str()
        .expect("")
        .to_string()
}
