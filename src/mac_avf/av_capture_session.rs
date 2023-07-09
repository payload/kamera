use objc::{self, *};
use objc_foundation::*;

object_struct!(AVCaptureSession);
impl IAVCaptureSession for AVCaptureSession {}

pub trait IAVCaptureSession: INSObject {
    fn begin_configuration(&self) {
        unsafe { msg_send!(self, beginConfiguration) }
    }

    fn start_running(&self) {
        unsafe { msg_send!(self, startRunning) }
    }
}
