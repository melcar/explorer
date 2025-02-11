mod f {
    use std::fs;
    use std::io;
    use std::path::Path;

    fn visit_dirs_intern(dir: &Path, depth: u8, cb: &dyn Fn(&fs::DirEntry, u8)) -> io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    visit_dirs_intern(&path, depth + 1, cb)?;
                }
                cb(&entry, depth);
            }
        }
        Ok(())
    }

    fn print_file(entry: &fs::DirEntry, depth: u8) {
        println!(
            "{} {}",
            (0..depth * 3).map(|_| '-').collect::<String>(),
            entry
                .file_name()
                .into_string()
                .unwrap_or("Not unwrapped".to_string())
        );
    }

    pub fn visit_dirs(path: &str) {
        let _ = visit_dirs_intern(Path::new(path), 0, &print_file);
    }
}

fn main() {
    let _ = f::visit_dirs(".");
}
