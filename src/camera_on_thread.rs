use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread::JoinHandle,
};

use crate::{Camera, CameraInfo};

enum Message {
    NewDefaultDevice,
    EnumerateCameras,
    Start,
    Stop,
    WaitForFrame,
    ChangeDevice,
}

enum Response {
    EnumerateCameras(Vec<CameraInfo>),
    WaitForFrame(((u32, u32), Vec<u8>)),
}

pub struct CameraOnThread {
    messages: Sender<Message>,
    responses: Receiver<Response>,
    thread_join: JoinHandle<()>,
}

impl CameraOnThread {
    pub fn new_default_device() -> Self {
        let (messages, message_rx) = channel::<Message>();
        let (response_tx, responses) = channel::<Response>();
        let thread_join = std::thread::spawn(move || {
            use Message::*;
            let mut camera = None;
            let mut send_result = Ok(());
            while let (Ok(message), Ok(())) = (message_rx.recv(), &send_result) {
                match message {
                    NewDefaultDevice => {
                        camera = Some(Camera::new_default_device());
                    }
                    EnumerateCameras => {
                        send_result = response_tx
                            .send(Response::EnumerateCameras(Camera::enumerate_cameras()));
                    }
                    Start => {
                        camera.as_ref().map(|c| c.start());
                    }
                    Stop => {
                        camera.as_ref().map(|c| c.stop());
                    }
                    WaitForFrame => {
                        if let Some(ref camera) = camera {
                            if let Some(frame) = camera.wait_for_frame() {
                                let size = frame.size_u32();
                                let pixels = frame.data().data_u8().to_vec();
                                let response = Response::WaitForFrame((size, pixels));
                                send_result = response_tx.send(response);
                            }
                        }
                    }
                    ChangeDevice => {
                        camera.as_mut().map(|c| c.change_device());
                    }
                }
            }
            if let Some(camera) = camera {
                camera.stop();
            }
        });

        messages.send(Message::NewDefaultDevice).unwrap();

        CameraOnThread { messages, responses, thread_join }
    }

    pub fn enumerate_cameras() -> Vec<CameraInfo> {
        std::thread::spawn(|| Camera::enumerate_cameras()).join().unwrap()
    }

    pub fn start(&self) {
        self.messages.send(Message::Start).unwrap();
    }

    pub fn stop(&self) {
        self.messages.send(Message::Stop).unwrap();
    }

    pub fn change_device(&mut self) {
        self.messages.send(Message::ChangeDevice).unwrap();
    }

    pub fn wait_for_frame(&self) -> Option<((u32, u32), Vec<u8>)> {
        self.messages.send(Message::WaitForFrame).unwrap();
        if let Response::WaitForFrame((size, pixels)) = self.responses.recv().unwrap() {
            return Some((size, pixels));
        } else {
            todo!();
        }
    }
}
