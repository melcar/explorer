use explorer::file_explorer;
use file_explorer::FileExplorer;

fn main() -> iced::Result {
    iced::run("File explorer", FileExplorer::update, FileExplorer::view)
}
