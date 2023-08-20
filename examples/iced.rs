use std::iter::repeat;
use std::time::Instant;

use iced::widget::{column, container, image, text};
use iced::{
    executor, subscription, window, Application, Command, Element, Event, Length, Settings,
    Subscription, Theme,
};
use kamera::{CameraInfo, CameraOnThread};

pub fn main() -> iced::Result {
    Example::run(Settings::default())
}

struct Example {
    camera: CameraOnThread,
    cameras: Vec<CameraInfo>,
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
        let camera = CameraOnThread::new_default_device();
        let cameras = CameraOnThread::enumerate_cameras();
        camera.start();

        let app = Self {
            camera,
            cameras,
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
                if let Some(((w, h), pixels)) = self.camera.wait_for_frame() {
                    let pixels = rgba_to_bgra(w, h, &pixels);
                    self.current_frame = image::Handle::from_pixels(w, h, pixels);
                }
            }
            Message::Event(iced::event::Event::Mouse(mouse_event)) => {
                if let iced::mouse::Event::ButtonPressed(_) = mouse_event {
                    self.camera.change_device();
                }
            }
            _ => {}
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let camera_frame = image(self.current_frame.clone());
        let camera_label = text(self.cameras.get(0).map(|c| c.label.clone()).unwrap_or_default());
        container(column!(camera_frame, camera_label))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

fn rgba_to_bgra(w: u32, h: u32, pixels: &[u8]) -> Vec<u8> {
    use ffimage::color::*;
    use ffimage::packed::*;
    use ffimage::traits::Convert;
    let a = ImageView::<Rgba<u8>>::from_buf(pixels, w, h).unwrap();
    let mut b = ImageBuffer::<Bgra<u8>>::new(w, h, 0u8);
    a.convert(&mut b);
    b.into_buf()
}
