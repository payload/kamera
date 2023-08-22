use std::iter::repeat;
use std::time::Instant;

use iced::widget::{column, container, image, text};
use iced::{
    executor, subscription, window, Application, Command, Element, Event, Length, Settings,
    Subscription, Theme,
};
use kamera::camera_idea::*;

pub fn main() -> iced::Result {
    Example::run(Settings::default())
}

struct Example {
    camera: Camera,
    current_frame: image::Handle,
}

#[derive(Debug, Clone)]
#[allow(clippy::enum_variant_names)]
enum Message {
    Tick(Instant),
    Event(Event),
}

impl Application for Example {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        let camera = Camera::new();
        camera.play();

        let app = Self {
            camera,
            current_frame: image::Handle::from_pixels(
                16,
                16,
                Vec::from_iter(repeat([0, 0, 0, 0]).take(16 * 16).flatten()),
            ),
        };

        (app, Command::none())
    }

    fn title(&self) -> String {
        String::from("Custom widget - Iced")
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(
            [subscription::events().map(Message::Event), window::frames().map(Message::Tick)]
                .into_iter(),
        )
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick(_instant) => {
                for event in self.camera.events() {
                    match event {
                        CameraEvent::Frame => {
                            let frame = self.camera.current_frame();
                            self.current_frame = image::Handle::from_pixels(
                                frame.width(),
                                frame.height(),
                                frame.pixels().to_vec(),
                            );
                        }
                        CameraEvent::StateChange => {}
                    }
                }
            }
            Message::Event(iced::event::Event::Mouse(mouse_event)) => {
                if let iced::mouse::Event::ButtonPressed(_) = mouse_event {
                    self.camera.change_camera(CameraChange::Next);
                }
            }
            _ => {}
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let camera_frame = image(self.current_frame.clone());
        let camera_label = text(&self.camera.info().label);
        container(column!(camera_frame, camera_label))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
