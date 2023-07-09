use objc::{self, *};
use objc_foundation::*;
use objc_id::*;

use super::AVCaptureDeviceFormat;

object_struct!(AVCaptureDevice);
impl IAVCaptureDevice for AVCaptureDevice {}

pub trait IAVCaptureDevice: INSObject {
    fn default_video_device() -> Id<Self> {
        let cls = Self::class();
        let video = Self::media_type_video();
        unsafe { Id::from_ptr(msg_send![cls, defaultDeviceWithMediaType: video]) }
    }

    fn all_video_devices() -> Id<NSArray<AVCaptureDevice>> {
        let cls = Self::class();
        let video = Self::media_type_video();
        unsafe { Id::from_ptr(msg_send!(cls, devicesWithMediaType: video)) }
    }

    fn media_type_video() -> Id<NSString> {
        NSString::from_str("vide")
    }

    fn unique_id(&self) -> Id<NSString> {
        unsafe { Id::from_retained_ptr(msg_send!(self, uniqueID)) }
    }

    fn localized_name(&self) -> Id<NSString> {
        unsafe { Id::from_retained_ptr(msg_send!(self, localizedName)) }
    }

    fn formats(&self) -> Id<NSArray<AVCaptureDeviceFormat>> {
        unsafe { Id::from_ptr(msg_send![self, formats]) }
    }
}
