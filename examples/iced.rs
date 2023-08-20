use std::iter::repeat;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Instant;

use iced::widget::{column, container, image, text};
use iced::{
    executor, window, Alignment, Application, Command, Element, Length, Settings, Subscription,
    Theme,
};
use kamera::{Camera, CameraInfo, CameraOnThread};

pub fn main() -> iced::Result {
    Example::run(Settings::default())
}

struct Example {
    camera: CameraOnThread,
    cameras: Vec<CameraInfo>,
    current_frame: image::Handle,
}

#[derive(Debug, Clone, Copy)]
#[allow(clippy::enum_variant_names)]
enum Message {
    Tick(Instant),
}

impl Application for Example {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        let camera = CameraOnThread::new_default_device();
        let cameras = CameraOnThread::enumerate_cameras();
        camera.start();

        (
            Self {
                camera,
                cameras,
                current_frame: image::Handle::from_pixels(
                    16,
                    16,
                    Vec::from_iter(repeat([0, 0, 0, 0]).take(16 * 16).flatten()),
                ),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Custom widget - Iced")
    }

    fn subscription(&self) -> Subscription<Message> {
        window::frames().map(Message::Tick)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick(_instant) => {
                if let Some(((w, h), pixels)) = self.camera.wait_for_frame() {
                    self.current_frame = image::Handle::from_pixels(w, h, pixels);
                }
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let mut column = column(Vec::new());

        column = column.push(image(self.current_frame.clone()));

        for camera in self.cameras.iter() {
            column = column.push(text(&camera.label));
        }

        let content = column.padding(20).spacing(20).align_items(Alignment::Start);

        container(content).width(Length::Fill).height(Length::Fill).center_x().center_y().into()
    }
}
