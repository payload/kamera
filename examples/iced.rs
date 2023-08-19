use std::iter::repeat;
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::time::Instant;

use iced::widget::{column, container, image, text};
use iced::{
    executor, window, Alignment, Application, Command, Element, Length, Settings, Subscription,
    Theme,
};
use kamera::{Camera, CameraInfo};

pub fn main() -> iced::Result {
    Example::run(Settings::default())
}

struct Example {
    cameras: Vec<CameraInfo>,
    #[allow(unused)]
    end_camera: Sender<()>,
    camera_frame: Receiver<((u32, u32), Vec<u8>)>,
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
        let (end_camera, end) = channel::<()>();
        let (send_frame, camera_frame) = channel::<((u32, u32), Vec<u8>)>();
        let (send_cameras, cameras) = channel::<Vec<CameraInfo>>();
        std::thread::spawn(move || {
            let _ = send_cameras.send(Camera::enumerate_cameras());
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

        let cameras = cameras.recv().expect("enumerate cameras");

        (
            Self {
                cameras,
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
        match message {
            Message::Tick(_instant) => {
                while let Ok(((w, h), pixels)) = self.camera_frame.try_recv() {
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

        let content = column.padding(20).spacing(20).max_width(500).align_items(Alignment::Center);

        container(content).width(Length::Fill).height(Length::Fill).center_x().center_y().into()
    }
}
