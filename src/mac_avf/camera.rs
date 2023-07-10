use super::*;
use objc2::rc::Id;
use std::sync::Arc;

#[derive(Debug)]
pub struct Camera {
    device: Id<AVCaptureDevice>,
    input: Id<AVCaptureDeviceInput>,
    output: Id<AVCaptureVideoDataOutput>,
    session: Id<AVCaptureSession>,
    slot: Arc<Slot>,
}

#[derive(Debug)]
pub struct Frame {
    sample: SampleBuffer,
}

pub struct FrameData<'a> {
    pixels: Pixels<'a>,
}

impl Camera {
    pub fn new_default_device() -> Self {
        let device = AVCaptureDevice::default_video_device();
        let input = AVCaptureDeviceInput::from_device(&device).unwrap();
        let output = AVCaptureVideoDataOutput::new();
        output.set_video_settings(&video_settings_from_pixel_format("ARGB"));
        let delegate = SampleBufferDelegate::new();
        let slot = delegate.slot();
        let session = AVCaptureSession::new();
        output.set_sample_buffer_delegate(delegate);
        session.add_input(&input);
        session.add_output(&output);

        Camera { device, input, output, session, slot }
    }

    pub fn start(&self) {
        self.session.start_running();
    }

    pub fn stop(&self) {
        self.session.stop_running();
    }

    pub fn wait_for_frame(&self) -> Option<Frame> {
        self.slot.wait_for_sample().map(|sample| Frame { sample })
    }
}

impl Frame {
    pub fn data(&self) -> FrameData {
        FrameData { pixels: self.sample.pixels() }
    }

    pub fn size_u32(&self) -> (u32, u32) {
        let (w, h) = self.sample.size_usize();
        (w as _, h as _)
    }
}

impl<'a> FrameData<'a> {
    pub fn data_u8(&self) -> &[u8] {
        self.pixels.data
    }

    pub fn data_u32(&self) -> &[u32] {
        self.pixels.u32
    }
}
