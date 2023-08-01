use super::*;
use objc2::rc::Id;
use std::sync::Arc;

#[derive(Debug)]
pub struct Camera {
    device: Id<AVCaptureDevice>,
    input: Id<AVCaptureDeviceInput>,
    #[allow(unused)]
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

    pub fn change_device(&mut self) {
        let devices = AVCaptureDevice::all_video_devices();
        let Some(index) = devices.iter().position(|d| d.unique_id() == self.device.unique_id())
        else {
            return;
        };
        let new_index = (index + 1) % devices.len();
        if new_index == index {
            return;
        }
        let new_device = devices[new_index].retain();
        let new_input = AVCaptureDeviceInput::from_device(&new_device).unwrap();
        self.session.remove_input(&self.input);
        self.device = new_device;
        self.input = new_input;
        self.session.add_input(&self.input);
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

#[cfg(test)]
const TEST_FRAMES: usize = 3;

#[test]
fn change_device() {
    let mut camera = Camera::new_default_device();
    camera.start();

    std::iter::from_fn(|| camera.wait_for_frame())
        .map(|s| println!("{s:?}"))
        .take(TEST_FRAMES)
        .count();

    camera.change_device();

    std::iter::from_fn(|| camera.wait_for_frame())
        .map(|s| println!("{s:?}"))
        .take(TEST_FRAMES)
        .count();
}
