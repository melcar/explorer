mod f {
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
}

#[derive(Default)]
struct Counter {
    value: Vec<String>,
}
#[derive(Debug, Clone, Copy)]
pub enum Message {
    Load,
    Click,
}
use iced::widget::{button, column, row, text, Column, Row};
use iced::Element;

impl Counter {
    pub fn view(&self) -> Column<Message> {
        let rows: Vec<Element<Message>> =
            self.value.iter().map(|x| row![button((*x).as_str()).on_press(Message::Click)].into()).collect();
        // We use a column: a simple vertical layout
        column![
            row![button("Load").on_press(Message::Load)],
            column![Column::from_vec(rows)]
        ]
    }
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Load => {
                self.value = f::visit_dirs(".")
                    .unwrap_or_default()
                    .iter()
                    .map(|x| x.file_name().to_str().unwrap_or("hoho").to_string())
                    .collect();
            }
            Message::Click=> (),
        }
    }
}

fn main() -> iced::Result {
    if let Ok(files) = f::visit_dirs(".") {
        files.iter().for_each(f::print_file);
    }
    iced::run("A cool counter", Counter::update, Counter::view)
}
