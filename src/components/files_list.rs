// UI state view and action taht represents the listing of files/Directories
use crate::components::files_actions::{get_file_name, get_file_size, visit_dirs};
use iced::widget::{button, column, image, row, scrollable, text, Column};
use iced::Element;
use std::env;
use std::fs::{self, DirEntry};
use std::path::PathBuf;

use super::state::Message;

pub struct FilesList {
    value: Vec<fs::DirEntry>,
    current_directory: PathBuf,
    name_column_width: u16,
    file_size_column_width: u16,
}

impl Default for FilesList {
    fn default() -> Self {
        let path = env::current_dir().expect("");
        FilesList {
            value: visit_dirs(&path).unwrap_or_default(),
            current_directory: path,
            name_column_width: 250,
            file_size_column_width: 150,
        }
    }
}

enum EntryType<'a> {
    Directory(&'a DirEntry),
    File(&'a DirEntry),
}
fn make_file_list_row(entry: &DirEntry, width: u16) -> (EntryType, Element<Message>) {
    match entry.path().is_dir() {
        true => (
            EntryType::Directory(entry),
            text("").width(width).into(), // empty in row
        ),
        false => (
            EntryType::File(entry),
            text(get_file_size(entry).to_string()).width(width).into(),
        ),
    }
}

fn make_button(e: &DirEntry, width: u16) -> Element<'static, Message> {
    button(
        row![
            image("ressources/folder.png").width(30).height(20),
            text(get_file_name(e))
        ]
        .spacing(10),
    )
    .width(300)
    .on_press(Message::Click(e.path().to_string_lossy().to_string()))
    .width(width)
    .into()
}
fn make_text(e: &DirEntry, width: u16) -> Element<'static, Message> {
    text(get_file_name(e)).width(width).into()
}

fn make_name(e: EntryType, width: u16) -> Element<'static, Message> {
    match e {
        EntryType::Directory(name) => make_button(name, width),
        EntryType::File(name) => make_text(name, width),
    }
}

impl FilesList {
    pub fn view(&self) -> Column<Message> {
        let files_rows: Vec<_> = self
            .value
            .iter()
            .map(|x| make_file_list_row(x, self.file_size_column_width))
            .map(|x| row![make_name(x.0, self.name_column_width), x.1].into())
            .collect();
        column![
            button(image("ressources/goUp.png"))
                .on_press(Message::GoBack)
                .width(75)
                .height(75),
            scrollable(Column::from_vec(files_rows))
        ]
    }

    pub fn refresh(&mut self) {
        self.value = visit_dirs(&self.current_directory).unwrap_or_default();
    }

    pub fn visit(&mut self, directory: &str) {
        self.current_directory.push(directory);
        self.refresh()
    }

    pub fn go_back(&mut self) {
        self.current_directory = self
            .current_directory
            .parent()
            .unwrap_or(self.current_directory.as_path())
            .to_path_buf();
        self.refresh();
    }
}
