use icrate::Foundation::{NSArray, NSObjectProtocol, NSString};
use objc2::rc::Id;
use objc2::runtime::NSObject;
use objc2::{extern_class, msg_send_id, mutability, ClassType};

use super::AVCaptureDeviceFormat;

extern_class! {
    #[derive(PartialEq, Eq, Hash, Debug)]
    pub struct AVCaptureDevice;

    unsafe impl ClassType for AVCaptureDevice {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
    }
}

unsafe impl NSObjectProtocol for AVCaptureDevice {}

#[allow(unused)]
impl AVCaptureDevice {
    pub fn default_video_device() -> Id<Self> {
        let video = Self::media_type_video();
        unsafe { msg_send_id![Self::class(), defaultDeviceWithMediaType: &*video] }
    }

    pub fn all_video_devices() -> Id<NSArray<AVCaptureDevice>> {
        let video = Self::media_type_video();
        unsafe { msg_send_id!(Self::class(), devicesWithMediaType: &*video) }
    }

    pub fn media_type_video() -> Id<NSString> {
        NSString::from_str("vide")
    }

    pub fn unique_id(&self) -> Id<NSString> {
        unsafe { msg_send_id!(self, uniqueID) }
    }

    pub fn localized_name(&self) -> Id<NSString> {
        unsafe { msg_send_id!(self, localizedName) }
    }

    pub fn formats(&self) -> Id<NSArray<AVCaptureDeviceFormat>> {
        unsafe { msg_send_id![self, formats] }
    }
}

#[test]
fn default_video_device() {
    let device = AVCaptureDevice::default_video_device();
    println!("{device:#?}");
}

#[test]
fn all_video_devices() {
    let devices = AVCaptureDevice::all_video_devices();
    println!("{:#?}", devices.to_vec());
    assert!(devices.count() > 0);
}

#[test]
fn unique_id() {
    for device in AVCaptureDevice::all_video_devices().to_vec() {
        println!("{}", device.unique_id());
        assert!(device.unique_id().len() > 0);
    }
}

#[test]
fn localized_name() {
    for device in AVCaptureDevice::all_video_devices().to_vec() {
        println!("{}", device.localized_name());
        assert!(device.localized_name().len() > 0);
    }
}

#[test]
fn formats() {
    for device in AVCaptureDevice::all_video_devices().to_vec() {
        println!("{:#?}", device.formats().to_vec());
        assert!(device.formats().count() > 0);
    }
}
