mod attributes;
mod media_type;
mod source_reader_flag;
#[cfg(test)]
mod tests;
mod video_format;
mod video_frame;
mod win_mf;

pub use media_type::*;
pub use video_format::*;
pub use video_frame::*;
pub use win_mf::*;

pub struct Camera {
    camera: win_mf::Camera,
}

pub struct Frame {
    frame: win_mf::CameraFrame,
}

pub struct FrameData<'a> {
    data: &'a [u8],
}

impl Camera {
    pub fn new_default_device() -> Self {
        Camera { camera: win_mf::Camera::new_default_device() }
    }

    pub fn start(&self) {
        self.camera.start();
    }

    pub fn stop(&self) {
        self.camera.stop();
    }

    pub fn wait_for_frame(&self) -> Option<Frame> {
        self.camera.wait_for_next_frame().map(|frame| Frame { frame })
    }
}

impl Frame {
    pub fn data(&self) -> FrameData {
        FrameData { data: self.frame.data() }
    }

    pub fn size_u32(&self) -> (u32, u32) {
        self.frame.size_u32()
    }
}

impl<'a> FrameData<'a> {
    pub fn u32_data(&self) -> &[u32] {
        let (a, data, b) = unsafe { self.data.align_to() };
        debug_assert!(a.is_empty());
        debug_assert!(b.is_empty());
        data
    }
}
