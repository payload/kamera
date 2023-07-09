mod av_capture_device;
mod av_capture_device_format;
mod av_capture_device_input;
mod av_capture_session;
mod av_capture_video_data_output;

pub use objc_foundation::{INSArray, INSCopying, INSObject, INSString};
pub use objc_id::Id;

pub use av_capture_device::*;
pub use av_capture_device_format::*;
pub use av_capture_device_input::*;
pub use av_capture_session::*;
pub use av_capture_video_data_output::*;

#[link(name = "AVFoundation", kind = "framework")]
extern "C" {}
