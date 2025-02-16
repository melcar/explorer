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
}

impl Default for FilesList {
    fn default() -> Self {
        let path = env::current_dir().expect("");
        FilesList {
            value: visit_dirs(&path).unwrap_or_default(),
            current_directory: path,
        }
    }
}

enum EntryType<'a> {
    Directory(&'a DirEntry),
    File(&'a DirEntry),
}
fn make_file_list_row(entry: &DirEntry) -> (EntryType, Element<Message>) {
    match entry.path().is_dir() {
        true => (
            EntryType::Directory(entry),
            text("").into(), // empty in row
        ),
        false => (
            EntryType::File(entry),
            text(get_file_size(entry).to_string()).into(),
        ),
    }
}

fn make_button(e: &DirEntry) -> Element<'static, Message> {
    button(
        row![
            image("ressources/folder.png").width(30).height(20),
            text(get_file_name(e))
        ]
        .spacing(10),
    )
    .on_press(Message::Click(e.path().to_string_lossy().to_string()))
    .into()
}
fn make_text(e: &DirEntry) -> Element<'static, Message> {
    text(get_file_name(e)).into()
}

fn make_name(e: EntryType) -> Element<'static, Message> {
    match e {
        EntryType::Directory(name) => make_button(name),
        EntryType::File(name) => make_text(name),
    }
}

impl FilesList {
    pub fn view(&self) -> Column<Message> {
        let (file_name_column, file_size_colunm): (Vec<_>, Vec<_>) =
            self.value.iter().map(make_file_list_row).unzip();
        column![
            button(image("ressources/goUp.png"))
                .on_press(Message::GoBack)
                .width(75)
                .height(75),
            scrollable(
                row![
                    column![Column::from_vec(
                        file_name_column.into_iter().map(make_name).collect()
                    )],
                    column![Column::from_vec(file_size_colunm)]
                ]
                .width(1000)
            )
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
