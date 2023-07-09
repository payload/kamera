mod av_capture_device;
mod av_capture_device_format;
mod av_capture_device_input;
mod av_capture_session;
mod av_capture_video_data_output;
mod sample_buffer;
mod sample_buffer_delegate;
mod video_output_settings;
#[cfg(test)]
mod test_scenarios;
#[cfg(test)]
mod reflect_class;

pub use objc2::*;

pub use av_capture_device::*;
pub use av_capture_device_format::*;
pub use av_capture_device_input::*;
pub use av_capture_session::*;
pub use av_capture_video_data_output::*;
pub use sample_buffer::*;
pub use sample_buffer_delegate::*;
pub use video_output_settings::*;

#[link(name = "AVFoundation", kind = "framework")]
extern "C" {}
