use std::ptr::null_mut;

use objc::{self, *};
use objc_foundation::*;
use objc_id::*;

use super::AVCaptureDevice;

object_struct!(AVCaptureDeviceInput);
impl IAVCaptureDeviceInput for AVCaptureDeviceInput {}

pub trait IAVCaptureDeviceInput: INSObject {
    fn from_device(device: &AVCaptureDevice) -> Option<Id<Self>> {
        unsafe {
            let error = null_mut::<()>();
            let input: *mut Self =
                msg_send!(Self::class(), deviceInputWithDevice: device error: error); // TODO what is the difference to initWithDevice ??
            if error.is_null() && !input.is_null() {
                Some(Id::from_ptr(input))
            } else {
                None // TODO visible error
            }
        }
    }
}
