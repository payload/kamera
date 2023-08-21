use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, RwLock};
use std::thread::JoinHandle;

pub struct Camera {
    current_frame: RwLock<Arc<CameraFrame>>,
    state: RwLock<Arc<CameraState>>,
    event: Option<Receiver<CameraEvent>>,    
}

pub struct CameraState {
    state: State,
    raw_format: RawFormat,
    constraints: CameraConstraints,
    capabilities: CameraCapabilities,
    setting: CameraSettings,
    camera_info: Arc<CameraInfo>,
    available_cameras: Vec<CameraInfo>,
}

pub struct CameraFrame {
    frame_counter: usize,
    width: u32,
    height: u32,
    raw_format: RawFormat,
    raw_data: Vec<u8>,
    camera_info: Arc<CameraInfo>,
}

pub enum RawFormat {
    ARGB,
    BGRA,
}

pub enum State {
    NoCamera,
    Playing,
    Paused,
    Stopped,
    Error,
}

pub enum CameraEvent {
    Frame,
    StateChange,
}

pub enum CameraChange {
    Next,
}

pub struct CameraInfo {
    pub id: String,
    pub label: String,
}

pub struct CameraConstraints {
    pub width: Constraint<u32>,
    pub height: Constraint<u32>,
    pub frame_rate: Constraint<u32>,
}

pub struct CameraCapabilities {
    pub width: Capability<u32>,
    pub height: Capability<u32>,
    pub frame_rate: Capability<u32>,
}

pub struct CameraSettings {
    pub width: u32,
    pub height: u32,
    pub frame_rate: u32,
}

pub enum Constraint<T> {
    Any,
    Pref(T),
}
pub enum Capability<T> {
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

    pub fn events(&mut self) -> Option<Receiver<CameraEvent>> {
        self.event.take()
    }
}
