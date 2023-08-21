mod camera;
pub use camera::*;

mod camera_on_thread;
pub use camera_on_thread::*;

#[cfg(target_os = "macos")]
pub(crate) mod mac_avf;

#[cfg(target_os = "windows")]
pub(crate) mod win_mf;

#[cfg(target_os = "linux")]
pub(crate) mod linux_v4l2;

/*
Camera
    current frame + frame id, so I can use it in a widget now or pull it into encoding when I am ready for it
    new frame notifier, so I can wait for the next new frame or get canceled when there will be no new frame
    
    // maybe part of settings
    device label, so I can use it in UI
    device id, so I can use it as a handle for other APIs

    state: no camera, playing, pauseed, stopped, error
    state change notifier
    error notifier ?

    play, so I get incoming frames
    pause, so I can keep the device but stop incoming frames
    stop, so I can stop the device and release it (TODO make current frame black pr empty?)
    change camera, so playing state is preserved but device change is easy

    ABOUT width, height, aspect_ratio, frame_rate

    camera constraints (only support prefered)
    camera capabilities (only min max range)
    camera settings (to tell what was chosen)

    raw_format, so I can choose what I need given my renderer or encoder needs
        ARGB, BGRA, RGB, BGR (later NV12, I420, handle for platform specific way)

    current list of camera descriptors, so it is easy to show a list of cameras
    list of cameras change notifier

    detail information about camera, but it is very platform specific, so needs a dynamic structure

CameraFrame
    frame_counter
    raw_data
    raw_format (since it can change)
    width, height
    aspect_ratio (pre calculated?)
    frame_rate (at that time or frame_duration)
    device id (so the source is known exactly)
    device label (for good measure? maybe just device info)
*/
