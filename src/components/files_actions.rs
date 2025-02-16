// funciton that can help with updating states' compoennts

use std::fmt::Display;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::time::SystemTime;

pub enum FileSize {
    B(f32),
    KB(f32),
    MB(f32),
    GB(f32),
}

impl FileSize {
    fn new(v: u64) -> Self {
        match v {
            v if v < 1000 => FileSize::B(v as f32),
            v if v < 1_000_000 => FileSize::KB((v as f32) / 1000f32),
            v if v < 1_000_000_000 => FileSize::MB((v as f32) / 1_000_000f32),
            v => FileSize::GB((v / 1_000_000_000) as f32), //todo also get the comma
        }
    }

    fn value(&self) -> f32 {
        match self {
            FileSize::B(s) => *s,
            FileSize::KB(s) => *s,
            FileSize::MB(s) => *s,
            FileSize::GB(s) => *s,
        }
    }

    fn size_type(&self) -> String {
        match self {
            FileSize::B(_) => "B".to_string(),
            FileSize::KB(_) => "KB".to_string(),
            FileSize::MB(_) => "MB".to_string(),
            FileSize::GB(_) => "GB".to_string(),
        }
    }
}

impl Display for FileSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.value(), self.size_type())
    }
}

pub fn visit_dirs(dir: &PathBuf) -> io::Result<Vec<fs::DirEntry>> {
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

pub fn get_file_size(entry: &fs::DirEntry) -> FileSize {
    match entry.metadata() {
        Ok(metadata) => FileSize::new(metadata.len()),
        _ => FileSize::new(0),
    }
}

pub fn get_file_last_edit(entry: &fs::DirEntry) -> SystemTime {
    if let Ok(metadata) = entry.metadata() {
        return metadata.modified().unwrap();
    }
    SystemTime::now()
}
