use objc::{self, *};
use objc_foundation::*;

use super::{IAVCaptureDeviceInput, IAVCaptureOutput};

object_struct!(AVCaptureSession);
impl IAVCaptureSession for AVCaptureSession {}

pub trait IAVCaptureSession: INSObject {
    fn begin_configuration(&self) {
        unsafe { msg_send!(self, beginConfiguration) }
    }

    fn start_running(&self) {
        unsafe { msg_send!(self, startRunning) }
    }

    fn stop_running(&self) {
        unsafe { msg_send!(self, stopRunning) }
    }

    fn add_input(&self, input: &impl IAVCaptureDeviceInput) {
        unsafe { msg_send!(self, addInput: input) }
    }

    fn add_output(&self, output: &impl IAVCaptureOutput) {
        unsafe { msg_send!(self, addOutput: output) }
    }
}
