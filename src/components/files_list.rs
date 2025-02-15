// UI state view and action taht represents the listing of files/Directories
use crate::components::files_actions::{get_file_name, visit_dirs};
use iced::widget::{button, column, row, text, Column};
use iced::Element;
use std::fs;

use super::state::Message;

#[derive(Default)]
pub struct FilesList {
    value: Vec<fs::DirEntry>,
}

impl FilesList {
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
    pub fn load(&mut self) {
        self.value = visit_dirs(".").unwrap_or_default();
    }

    pub fn visit(&mut self, directory: &str) {
        self.value = visit_dirs(directory).unwrap_or_default();
    }
}
