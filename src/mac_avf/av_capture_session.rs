use icrate::Foundation::NSObjectProtocol;
use objc2::rc::Id;
use objc2::runtime::NSObject;
use objc2::{extern_class, msg_send, msg_send_id, mutability, ClassType};

use super::{AVCaptureDeviceInput, AVCaptureVideoDataOutput};

extern_class! {
    #[derive(PartialEq, Eq, Hash, Debug)]
    pub struct AVCaptureSession;

    unsafe impl ClassType for AVCaptureSession {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
    }
}

unsafe impl NSObjectProtocol for AVCaptureSession {}

impl AVCaptureSession {
    pub fn new() -> Id<Self> {
        unsafe { msg_send_id!(Self::class(), new) }
    }

    pub fn begin_configuration(&self) {
        unsafe { msg_send!(self, beginConfiguration) }
    }

    pub fn start_running(&self) {
        unsafe { msg_send!(self, startRunning) }
    }

    pub fn stop_running(&self) {
        unsafe { msg_send!(self, stopRunning) }
    }

    pub fn add_input(&self, input: &AVCaptureDeviceInput) {
        unsafe { msg_send!(self, addInput: input) }
    }

    pub fn add_output(&self, output: &AVCaptureVideoDataOutput) {
        unsafe { msg_send!(self, addOutput: output) }
    }
}

#[test]
fn new() {
    println!("{:?}", AVCaptureSession::new());
}

#[test]
fn start_running() {
    AVCaptureSession::new().start_running();
}

#[test]
fn stop_running() {
    AVCaptureSession::new().stop_running();
}

#[test]
fn begin_configuration() {
    AVCaptureSession::new().begin_configuration();
}

#[test]
fn add_input() {
    use super::AVCaptureDevice;
    let device = AVCaptureDevice::default_video_device();
    let input = AVCaptureDeviceInput::from_device(&device).unwrap();
    AVCaptureSession::new().add_input(&*input);
}

#[test]
fn add_output() {
    let output = AVCaptureVideoDataOutput::new();
    AVCaptureSession::new().add_output(&*output);
}
