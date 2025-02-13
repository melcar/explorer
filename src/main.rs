use iced::widget::{button, column, row, text, Column, Row};
use iced::Element;
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

#[derive(Default)]
struct Counter {
    value: Vec<fs::DirEntry>,
}
#[derive(Debug, Clone)]
pub enum Message {
    Load,
    Click(String),
}

pub fn get_file_name(entry: &fs::DirEntry) -> String {
    entry
        .file_name()
        .as_os_str()
        .to_str()
        .expect("")
        .to_string()
}

impl Counter {
    pub fn view(&self) -> Column<Message> {
        let rows: Vec<Element<Message>> = self
            .value
            .iter()
            .map(|x| {
                row![button(text(get_file_name(x)))
                    .on_press(Message::Click(x.path().to_string_lossy().to_string()))
                    .width(300)]
                .into()
            })
            .collect();
        // We use a column: a simple vertical layout
        column![
            row![button("Load").on_press(Message::Load)],
            column![Column::from_vec(rows)]
        ]
    }
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Load => {
                self.value = visit_dirs(".").unwrap_or_default();
            }
            Message::Click(file_name) => {
                self.value = visit_dirs(&file_name).unwrap_or_default();
            }
        }
    }
}

fn main() -> iced::Result {
    iced::run("File explorer", Counter::update, Counter::view)
}
