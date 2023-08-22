mod camera;
pub use camera::*;

mod camera_on_thread;
pub use camera_on_thread::*;

pub mod camera_idea;

#[cfg(target_os = "macos")]
mod mac_avf;
#[cfg(target_os = "macos")]
pub(crate) use mac_avf as backend;

#[cfg(target_os = "windows")]
mod win_mf;
#[cfg(target_os = "windows")]
pub(crate) use win_mf as backend;

#[cfg(target_os = "linux")]
mod linux_v4l2;
#[cfg(target_os = "linux")]
pub(crate) use linux_v4l2 as backend;
