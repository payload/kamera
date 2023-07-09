use icrate::Foundation::{NSError, NSObjectProtocol};
use objc2::rc::Id;
use objc2::runtime::NSObject;
use objc2::{extern_class, msg_send_id, mutability, ClassType};

use super::AVCaptureDevice;

extern_class!(
    #[derive(PartialEq, Eq, Hash, Debug)]
    pub struct AVCaptureDeviceInput;

    unsafe impl ClassType for AVCaptureDeviceInput {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for AVCaptureDeviceInput {}

impl AVCaptureDeviceInput {
    pub fn from_device(device: &AVCaptureDevice) -> std::result::Result<Id<Self>, Id<NSError>> {
        unsafe { msg_send_id![Self::class(), deviceInputWithDevice: device, error: _] }
    }
}

#[test]
fn from_device() {
    let device = AVCaptureDevice::default_video_device();
    let input = AVCaptureDeviceInput::from_device(&device);
    println!("{input:?}");
    assert!(input.is_ok());
}
