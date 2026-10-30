use std::path::PathBuf;

use iced::{
    Length::{Fill, FillPortion},
    Theme, Widget,
    widget::{container, row, text},
};

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .theme(Theme::TokyoNight)
        .run()
}

#[derive(Default)]
struct App {
    files_hovered: Vec<PathBuf>,
    files_dropped: Vec<PathBuf>,
}

#[derive(Debug, Clone)]
enum Message {
    FileHoveredOnDropzone(PathBuf),
    FileDroppedOnDropzone(PathBuf),
    FilesHoveredLeftDropzone,
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::FileHoveredOnDropzone(path) => self.files_hovered.push(path),
            Message::FileDroppedOnDropzone(path) => {
                self.files_hovered.clear();
                self.files_dropped.push(path);
            }
            Message::FilesHoveredLeftDropzone => {
                self.files_hovered.clear();
            }
        }
    }

    fn view(&self) -> impl Widget<Message> {
        row![
            container("deadzone").width(FillPortion(1)).height(Fill),
            container("dropzone").width(FillPortion(1)).height(Fill)
        ]
        .width(Fill)
        .height(Fill)
    }
}
