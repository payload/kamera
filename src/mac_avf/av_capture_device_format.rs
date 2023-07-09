use objc_foundation::*;

object_struct!(AVCaptureDeviceFormat);
impl IAVCaptureDeviceFormat for AVCaptureDeviceFormat {}

pub trait IAVCaptureDeviceFormat: INSObject {}
