//Entry point of the UI. This is the main view.
//It also defines the layout of the different components and layout switches
use iced::widget::column;
use iced::Element;

use crate::components::files_list::FilesList;
use crate::components::state::Message;

#[derive(Default)]
pub struct FileExplorer {
    files_list: FilesList,
}

impl FileExplorer {
    pub fn view(&self) -> Element<Message> {
        let file_list = self.files_list.view();
        column![file_list].into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Load => self.files_list.load(),
            Message::Click(s) => self.files_list.visit(&s),
        }
    }
}
