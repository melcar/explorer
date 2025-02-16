// UI state view and action taht represents the listing of files/Directories
use crate::components::files_actions::{get_file_name, visit_dirs};
use iced::widget::{button, column, image, row, scrollable, text, Column};
use iced::Element;
use std::env;
use std::fs::{self};
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

impl FilesList {
    pub fn view(&self) -> Column<Message> {
        let rows: Vec<Element<Message>> = self
            .value
            .iter()
            .map(|x| -> Element<Message> {
                match x.path().is_dir() {
                    true => row![button(row![image("ressources/folder.png").height(30).width(30), text(get_file_name(x))])
                        .on_press(Message::Click(x.path().to_string_lossy().to_string()))],
                    false => row![text(get_file_name(x))],
                }
                .width(300)
                .into()
            })
            .collect();
        column![
            button(image("ressources/goUp.png"))
                .on_press(Message::GoBack)
                .width(75)
                .height(75),
            scrollable(column![Column::from_vec(rows)])
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
