use super::mf::*;

use std::{sync::mpsc::*, time::Duration};

use windows::Win32::Media::MediaFoundation::*;

#[allow(unused)]
#[derive(Debug)]
pub struct Camera {
    engine: IMFCaptureEngine,
    device: mf::Device,
    event_rx: Receiver<CaptureEngineEvent>,
    sample_rx: Receiver<Option<IMFSample>>,
    event_cb: IMFCaptureEngineOnEventCallback,
    sample_cb: IMFCaptureEngineOnSampleCallback,
}

#[derive(Debug)]
pub struct Frame {
    buffer: LockedBuffer,
}

pub struct FrameData<'a> {
    data: &'a [u8],
}

impl Camera {
    pub fn new_default_device() -> Self {
        co_initialize_multithreaded();
        media_foundation_startup().expect("media_foundation_startup");

        let engine = new_capture_engine().unwrap();
        let (event_tx, event_rx) = channel::<CaptureEngineEvent>();
        let (sample_tx, sample_rx) = channel::<Option<IMFSample>>();
        let event_cb = CaptureEventCallback { event_tx }.into();
        let sample_cb = CaptureSampleCallback { sample_tx }.into();

        let devices = mf::Device::enum_devices();
        let Some(device) = devices.first().cloned() else { todo!() };

        init_capture_engine(&engine, Some(&device.source), &event_cb).unwrap();

        let camera = Camera { engine, device, event_rx, sample_rx, event_cb, sample_cb };
        camera.wait_for_event(CaptureEngineEvent::Initialized);
        camera.prepare_source_sink();
        camera
    }

    pub fn start(&self) {
        unsafe { self.engine.StartPreview().unwrap() }
    }

    pub fn stop(&self) {
        capture_engine_stop_preview(&self.engine).unwrap();
    }

    pub fn wait_for_frame(&self) -> Option<Frame> {
        self.sample_rx
            // TODO sometimes running two engines on the same camera breaks frame delivery, so wait not too long
            .recv_timeout(Duration::from_secs(3))
            .ok()
            .flatten()
            .and_then(|sample| {
                let Some(mt) = capture_engine_sink_get_media_type(&self.engine).ok() else {
                    return None;
                };
                let width = mt.frame_width();
                let height = mt.frame_height();
                sample_to_locked_buffer(&sample, width, height).ok()
            })
            .map(|buffer: LockedBuffer| Frame { buffer })
    }

    pub fn change_device(&mut self) {
        let devices: Vec<mf::Device> =
            enum_device_sources().into_iter().map(mf::Device::new).collect();
        let Some(index) = devices.iter().position(|d| d.id() == self.device.id()) else { return };
        let new_index = (index + 1) % devices.len();

        if new_index == index {
            return;
        }
        let new_device = devices[new_index].clone();

        let engine = new_capture_engine().unwrap();
        let (event_tx, event_rx) = channel::<CaptureEngineEvent>();
        let (sample_tx, sample_rx) = channel::<Option<IMFSample>>();
        let event_cb = CaptureEventCallback { event_tx }.into();
        let sample_cb = CaptureSampleCallback { sample_tx }.into();

        init_capture_engine(&engine, Some(&new_device.source), &event_cb).unwrap();

        *self = Camera { engine, device: new_device, event_rx, sample_rx, event_cb, sample_cb };
        self.wait_for_event(CaptureEngineEvent::Initialized);
        self.prepare_source_sink();
        self.start(); // TODO watch out about playing state
    }
}

impl Camera {
    fn prepare_source_sink(&self) {
        capture_engine_prepare_sample_callback(&self.engine, &self.sample_cb).unwrap();
    }

    fn wait_for_event(&self, event: CaptureEngineEvent) {
        self.event_rx.iter().find(|e| e == &event);
    }
}

impl Frame {
    pub fn data(&self) -> FrameData {
        FrameData { data: self.buffer.data() }
    }

    pub fn size_u32(&self) -> (u32, u32) {
        (self.buffer.width, self.buffer.height)
    }
}

impl<'a> FrameData<'a> {
    pub fn data_u8(&self) -> &[u8] {
        self.data
    }

    pub fn data_u32(&self) -> &[u32] {
        let (a, data, b) = unsafe { self.data.align_to() };
        debug_assert!(a.is_empty());
        debug_assert!(b.is_empty());
        data
    }
}
