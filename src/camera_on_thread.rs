use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread::JoinHandle,
};

use crate::{Camera, CameraInfo};

enum Message {
    NewDefaultDevice,
    Start,
    Stop,
    WaitForFrame,
    ChangeDevice,
    Drop,
}

enum Response {
    WaitForFrame(((u32, u32), Vec<u8>)),
}

pub struct CameraOnThread {
    messages: Sender<Message>,
    responses: Receiver<Response>,
    thread_join: Option<JoinHandle<()>>,
}

impl CameraOnThread {
    pub fn new_default_device() -> Self {
        let (messages, message_rx) = channel::<Message>();
        let (response_tx, responses) = channel::<Response>();
        let thread_join = std::thread::spawn(move || {
            use Message::*;
            let mut camera = Camera::new_default_device();
            let mut send_result = Ok(());
            while let (Ok(message), Ok(())) = (message_rx.recv(), &send_result) {
                match message {
                    NewDefaultDevice => {
                        camera = Camera::new_default_device();
                    }
                    Start => {
                        camera.start();
                    }
                    Stop => {
                        camera.stop();
                    }
                    WaitForFrame => {
                        if let Some(frame) = camera.wait_for_frame() {
                            let size = frame.size_u32();
                            let pixels = frame.data().data_u8().to_vec();
                            let response = Response::WaitForFrame((size, pixels));
                            send_result = response_tx.send(response);
                        }
                    }
                    ChangeDevice => {
                        camera.change_device();
                    }
                    Drop => {
                        break;
                    }
                }
            }
            camera.stop();
        });

        messages.send(Message::NewDefaultDevice).unwrap();

        CameraOnThread { messages, responses, thread_join: Some(thread_join) }
    }

    pub fn enumerate_cameras() -> Vec<CameraInfo> {
        std::thread::spawn(Camera::enumerate_cameras).join().unwrap()
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
        match self.responses.recv().unwrap() {
            Response::WaitForFrame((size, pixels)) => Some((size, pixels)),
        }
    }
}

impl Drop for CameraOnThread {
    fn drop(&mut self) {
        if let Some(thread_join) = self.thread_join.take() {
            let _ = self.messages.send(Message::Drop);
            let _ = thread_join.join();
        }
    }
}
