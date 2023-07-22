use crate::{InnerCamera, InnerFrame, InnerFrameData};

#[derive(Debug)]
pub struct Camera {}

impl InnerCamera for Camera {
    type Frame = Frame;

    fn new_default_device() -> Self {
        todo!()
    }

    fn start(&self) {
        todo!()
    }

    fn stop(&self) {
        todo!()
    }

    fn wait_for_frame(&self) -> Option<Frame> {
        todo!()
    }
}

#[derive(Debug)]
pub struct Frame {}

impl InnerFrame for Frame {
    type FrameData = FrameData;

    fn data(&self) -> FrameData {
        todo!()
    }

    fn size_u32(&self) -> (u32, u32) {
        todo!()
    }
}

#[derive(Debug)]
pub struct FrameData {}

impl InnerFrameData for FrameData {
    fn data_u8(&self) -> &[u8] {
        todo!()
    }

    fn data_u32(&self) -> &[u32] {
        todo!()
    }
}
