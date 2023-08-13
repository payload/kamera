use std::iter::repeat;
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::time::Instant;

use iced::widget::{column, container, image, slider, text};
use iced::{
    executor, window, Alignment, Application, Command, Element, Length, Settings, Subscription,
    Theme,
};
use kamera::Camera;

pub fn main() -> iced::Result {
    Example::run(Settings::default())
}

struct Example {
    radius: [f32; 4],
    border_width: f32,
    #[allow(unused)]
    end_camera: Sender<()>,
    camera_frame: Receiver<((u32, u32), Vec<u8>)>,
    current_frame: image::Handle,
}

#[derive(Debug, Clone, Copy)]
#[allow(clippy::enum_variant_names)]
enum Message {
    RadiusTopLeftChanged(f32),
    RadiusTopRightChanged(f32),
    RadiusBottomRightChanged(f32),
    RadiusBottomLeftChanged(f32),
    BorderWidthChanged(f32),
    Tick(Instant),
}

impl Application for Example {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        let (end_camera, end) = channel::<()>();
        let (send_frame, camera_frame) = channel::<((u32, u32), Vec<u8>)>();
        std::thread::spawn(move || {
            let camera = Camera::new_default_device();
            camera.start();
            let mut keep_going = true;
            while keep_going {
                if Err(TryRecvError::Empty) != end.try_recv() {
                    keep_going = false;
                }
                if let Some(frame) = camera.wait_for_frame() {
                    let size = frame.size_u32();
                    let pixels = frame.data().data_u8().to_vec();
                    if send_frame.send((size, pixels)).is_err() {
                        keep_going = false;
                    }
                }
            }
            camera.stop();
        });

        (
            Self {
                radius: [50.0; 4],
                border_width: 0.0,
                end_camera,
                camera_frame,
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
        let [tl, tr, br, bl] = self.radius;

        match message {
            Message::Tick(instant) => {
                while let Ok(((w, h), pixels)) = self.camera_frame.try_recv() {
                    self.current_frame = image::Handle::from_pixels(w, h, pixels);
                    println!("frame. {}ms per frame", instant.elapsed().as_millis());
                }
            }
            Message::RadiusTopLeftChanged(radius) => {
                self.radius = [radius, tr, br, bl];
            }
            Message::RadiusTopRightChanged(radius) => {
                self.radius = [tl, radius, br, bl];
            }
            Message::RadiusBottomRightChanged(radius) => {
                self.radius = [tl, tr, radius, bl];
            }
            Message::RadiusBottomLeftChanged(radius) => {
                self.radius = [tl, tr, br, radius];
            }
            Message::BorderWidthChanged(width) => {
                self.border_width = width;
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let [tl, tr, br, bl] = self.radius;

        let content = column![
            image(self.current_frame.clone()),
            text(format!("Radius: {tl:.2}/{tr:.2}/{br:.2}/{bl:.2}")),
            slider(1.0..=100.0, tl, Message::RadiusTopLeftChanged).step(0.01),
            slider(1.0..=100.0, tr, Message::RadiusTopRightChanged).step(0.01),
            slider(1.0..=100.0, br, Message::RadiusBottomRightChanged).step(0.01),
            slider(1.0..=100.0, bl, Message::RadiusBottomLeftChanged).step(0.01),
            slider(1.0..=10.0, self.border_width, Message::BorderWidthChanged).step(0.01),
        ]
        .padding(20)
        .spacing(20)
        .max_width(500)
        .align_items(Alignment::Center);

        container(content).width(Length::Fill).height(Length::Fill).center_x().center_y().into()
    }
}
