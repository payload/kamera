use crate::backend;
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct Camera {
    current_frame: RwLock<Arc<CameraFrame>>,
    state: RwLock<Arc<CameraState>>,
    // event: Receiver<CameraEvent>,
    // backend_camera: RwLock<backend::Camera>,
}

#[derive(Debug)]
pub struct CameraState {
    state: State,
    raw_format: RawFormat,
    constraints: CameraConstraints,
    capabilities: CameraCapabilities,
    setting: CameraSettings,
    camera_info: Arc<CameraInfo>,
    available_cameras: Vec<CameraInfo>,
}

#[derive(Debug)]
pub struct CameraFrame {
    frame_counter: usize,
    width: u32,
    height: u32,
    raw_format: RawFormat,
    raw_data: Vec<u8>,
    camera_info: Arc<CameraInfo>,
}

#[derive(Debug)]
pub enum RawFormat {
    ARGB,
    BGRA,
    RGBA,
}

#[derive(Debug)]
pub enum State {
    NoCamera,
    Playing,
    Paused,
    Stopped,
    Error,
}

#[derive(Debug)]
pub enum CameraEvent {
    Frame,
    StateChange,
}

#[derive(Debug)]
pub enum CameraChange {
    Next,
}

#[derive(Default, Debug)]
pub struct CameraInfo {
    pub id: String,
    pub label: String,
}

#[derive(Default, Debug)]
pub struct CameraConstraints {
    pub width: Constraint<u32>,
    pub height: Constraint<u32>,
    pub frame_rate: Constraint<u32>,
}

#[derive(Default, Debug)]
pub struct CameraCapabilities {
    pub width: Capability<u32>,
    pub height: Capability<u32>,
    pub frame_rate: Capability<u32>,
}

#[derive(Default, Debug)]
pub struct CameraSettings {
    pub width: u32,
    pub height: u32,
    pub frame_rate: u32,
}

#[derive(Default, Debug)]
pub enum Constraint<T> {
    #[default]
    Any,
    Pref(T),
}

#[derive(Default, Debug)]
pub enum Capability<T> {
    #[default]
    Any, // TODO maybe not wanted
    Range(T, T),
}

////////////////////////////////////////////////

impl Camera {
    pub fn play(&self) {}
    pub fn pause(&self) {}
    pub fn stop(&self) {}
    pub fn change_camera(&self, _change: CameraChange) {}

    pub fn current_frame(&self) -> Arc<CameraFrame> {
        // TODO make an empty frame
        self.current_frame.read().unwrap().clone()
    }

    pub fn state(&self) -> Arc<CameraState> {
        // TODO make an error state
        self.state.read().unwrap().clone()
    }

    pub fn events(&self) -> impl Iterator<Item = CameraEvent> {
        if let Ok(mut frame) = self.current_frame.write() {
            let frame_counter = frame.frame_counter + 1;
            *frame = Arc::new(CameraFrame {
                frame_counter,
                ..CameraFrame::new_random_color(frame_counter)
            });
        }

        vec![CameraEvent::Frame, CameraEvent::StateChange].into_iter()
    }

    ////////////////////////////////////////////////

    pub fn new() -> Self {
        Self {
            current_frame: RwLock::new(Arc::new(CameraFrame::new())),
            state: RwLock::new(Arc::new(CameraState::new())),
        }
    }

    ////////////////////////////////////////////////

    pub fn info(&self) -> Arc<CameraInfo> {
        self.state.read().unwrap().camera_info.clone()
    }
}

impl CameraFrame {
    fn new() -> Self {
        Self {
            frame_counter: 0,
            width: 0,
            height: 0,
            raw_format: RawFormat::ARGB,
            raw_data: Vec::new(),
            camera_info: Arc::new(CameraInfo::default()),
        }
    }

    fn new_random_color(seed: usize) -> Self {
        let w = 160 * 2;
        let h = 90 * 2;

        // this should cycle through red, green, blue
        let f = (seed as f32) * 3.14 / 255.0;
        let r = (f * 5.0).sin().powi(2) * 255.0;
        let g = ((f + 0.8) * 5.0).sin().clamp(0.0, 1.0) * 255.0;
        let b = ((f + 1.6) * 5.0).sin().clamp(0.0, 1.0) * 255.0;
        Self {
            frame_counter: 0,
            width: w,
            height: h,
            raw_format: RawFormat::RGBA,
            camera_info: Arc::new(CameraInfo::default()),
            raw_data: std::iter::repeat([r as _, g as _, b as _, 255])
                .take((w * h) as _)
                .flatten()
                .collect(),
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn pixels(&self) -> &[u8] {
        &self.raw_data
    }
}

impl CameraState {
    fn new() -> Self {
        Self {
            state: State::NoCamera,
            raw_format: RawFormat::ARGB,
            constraints: CameraConstraints::default(),
            capabilities: CameraCapabilities::default(),
            setting: CameraSettings::default(),
            camera_info: Arc::new(CameraInfo::default()),
            available_cameras: Vec::new(),
        }
    }
}
