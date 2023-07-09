use std::{ffi::OsString, mem::MaybeUninit, sync::mpsc::*};

use windows::{
    core::*,
    Win32::{Media::MediaFoundation::*, System::Com::*},
};

use super::attributes::{mf_create_attributes, mf_get_string};
use super::media_type::MediaType;

#[derive(Clone, Debug)]
pub struct Device {
    activate: IMFActivate,
    source: IMFMediaSource,
}

impl Device {
    fn new(activate: IMFActivate) -> Self {
        let source = unsafe { activate.ActivateObject().unwrap() };
        Self { activate, source }
    }
}

impl PartialEq for Device {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        // unsafe { self.activate.ShutdownObject().unwrap() };
        println!("Device.drop done");
    }
}

impl Device {
    pub fn name(&self) -> String {
        mf_get_string(&self.activate, &MF_DEVSOURCE_ATTRIBUTE_FRIENDLY_NAME)
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|_| "NO NAME".into())
    }

    pub fn id(&self) -> OsString {
        let symlink = &MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_SYMBOLIC_LINK;
        mf_get_string(&self.activate, symlink).unwrap_or_else(|_| "NO ID".into())
    }

    pub fn query_media_types(&self) -> Vec<MediaType> {
        query_media_types_from_media_source(&self.source)
    }

    pub fn query_media_types_with_best_fps(&self) -> Vec<MediaType> {
        MediaType::filter_resolutions_with_max_fps(&self.query_media_types())
    }

    pub fn enum_devices() -> Vec<Device> {
        enum_device_sources().into_iter().map(Device::new).collect()
    }
}

pub struct Camera {
    frame: Option<CameraFrame>,
    engine: CaptureEngine,
}

impl Camera {
    pub fn new() -> Self {
        media_foundation_startup().expect("media_foundation_startup");
        let engine = CaptureEngine::new().unwrap();
        Self { engine, frame: None }
    }

    pub fn from_device(device: &Device) -> Self {
        media_foundation_startup().expect("media_foundation_startup");
        let mut engine = CaptureEngine::new().unwrap();
        engine.init(Some(device.clone())).unwrap();
        Self { engine, frame: None }
    }

    pub fn new_default_device() -> Self {
        let devices = Device::enum_devices();
        if let Some(device) = devices.first() {
            Self::from_device(device)
        } else {
            Self::new()
        }
    }

    pub fn device_name(&self) -> String {
        self.engine.device_name()
    }

    pub fn set_device(&mut self, device: &Device) {
        self.engine.init(Some(device.clone())).unwrap();
    }

    pub fn set_media_type(&mut self, media_type: &MediaType) {
        self.engine.set_media_type(media_type).unwrap();
    }

    pub fn start(&self) {
        self.engine.start_preview().unwrap();
    }

    pub fn just_start(&mut self) {
        self.engine.just_start_preview().unwrap();
    }

    pub fn stop(&self) {
        self.engine.stop_preview().unwrap();
    }

    pub fn get_frame(&mut self) -> Option<&CameraFrame> {
        if let Some(sample) = self.engine.try_recv_sample() {
            self.frame = Some(CameraFrame { sample });
        }

        self.frame.as_ref()
    }

    pub fn wait_frame(&mut self) -> Option<&CameraFrame> {
        if let Some(sample) = self.engine.recv_sample() {
            self.frame = Some(CameraFrame { sample });
        }

        self.frame.as_ref()
    }

    pub fn wait_for_next_frame(&self) -> Option<CameraFrame> {
        // TODO smash together with the other frame functions
        self.engine.recv_sample().map(|sample| CameraFrame { sample })
    }
}

impl Drop for Camera {
    fn drop(&mut self) {
        println!("Camera.drop");
        let _ = self.frame.take();
        println!("Camera.drop done");
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}

pub struct CameraFrame {
    sample: LockedBuffer,
}

impl CameraFrame {
    pub fn data(&self) -> &[u8] {
        self.sample.data()
    }

    pub fn width(&self) -> u32 {
        self.sample.width
    }

    pub fn height(&self) -> u32 {
        self.sample.height
    }

    pub fn size_u32(&self) -> (u32, u32) {
        (self.sample.width, self.sample.height)
    }
}

impl std::fmt::Display for CameraFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{} {}", self.sample.width, self.sample.height, self.sample.len)
    }
}

fn enum_device_sources() -> Vec<IMFActivate> {
    unsafe {
        let source_type = &MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE;
        let vidcap_guid = &MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_GUID;

        let mut count: u32 = 0;
        let mut activates: MaybeUninit<*mut Option<IMFActivate>> = MaybeUninit::uninit();
        let attributes = mf_create_attributes();
        attributes.SetGUID(source_type, vidcap_guid).unwrap();
        MFEnumDeviceSources(&attributes, activates.as_mut_ptr(), &mut count).unwrap();

        let activates = std::slice::from_raw_parts(activates.assume_init(), count as usize);
        let devices: Vec<_> = activates.iter().filter_map(|o| o.clone()).collect();
        CoTaskMemFree(Some(activates.as_ptr() as _));

        devices
    }
}

fn query_media_types_from_media_source(media_source: &IMFMediaSource) -> Vec<MediaType> {
    unsafe {
        let desc = media_source.CreatePresentationDescriptor().unwrap();

        let mut selected = false.into();
        let mut descriptor = None;
        desc.GetStreamDescriptorByIndex(0, &mut selected, &mut descriptor).unwrap();

        let desc = descriptor.unwrap();
        let mt_handler = desc.GetMediaTypeHandler().unwrap();

        let n = mt_handler.GetMediaTypeCount().unwrap();
        (0..n).map(|index| MediaType(mt_handler.GetMediaTypeByIndex(index).unwrap())).collect()
    }
}

fn media_foundation_startup() -> Result<()> {
    unsafe { MFStartup(MF_API_VERSION, MFSTARTUP_NOSOCKET) }
}

#[allow(unused)]
fn media_foundation_shutdown() -> Result<()> {
    unsafe { MFShutdown() }
}

struct CaptureEngine {
    device: Option<Device>,
    engine: IMFCaptureEngine,
    sample_media_type: Option<MediaType>,

    event_cb: IMFCaptureEngineOnEventCallback,
    sample_cb: IMFCaptureEngineOnSampleCallback,
    event_rx: Receiver<CaptureEngineEvent>,
    sample_rx: Receiver<Option<IMFSample>>,
}

impl Drop for CaptureEngine {
    fn drop(&mut self) {
        // media_foundation_shutdown().expect("media_foundation_shutdown");
        println!("CaptureEngine.drop done");
    }
}

impl CaptureEngine {
    fn new() -> Result<Self> {
        let engine = new_capture_engine()?;
        let (event_tx, event_rx) = channel();
        let (sample_tx, sample_rx) = channel();

        Ok(Self {
            device: None,
            engine,
            sample_media_type: None,
            event_rx,
            sample_rx,
            event_cb: CaptureEventCallback { event_tx }.into(),
            sample_cb: CaptureSampleCallback { sample_tx }.into(),
        })
    }

    fn init(&mut self, device: Option<Device>) -> Result<()> {
        self.device = device;
        let media_source = self.device.as_ref().map(|d| &d.source);
        init_capture_engine(&self.engine, media_source, &self.event_cb)?;
        self.wait_for_event(CaptureEngineEvent::Initialized);
        Ok(())
    }

    fn device_name(&self) -> String {
        self.device.clone().map(|d| d.name()).unwrap_or_default()
    }

    fn start_preview(&self) -> Result<()> {
        let sample_media_type = capture_engine_start_preview(&self.engine, &self.sample_cb)?;
        // self.sample_media_type = Some(sample_media_type);
        self.wait_for_event(CaptureEngineEvent::PreviewStarted);
        // PreviewStarted could be followed by Error before first Sample comes
        // First sample takes for example 250 ms but first Error could come alread 3 ms after PreviewStarted.
        // This is for example the case when the camera is already in use by another thread.
        Ok(())
    }

    fn just_start_preview(&mut self) -> Result<()> {
        unsafe { self.engine.StartPreview() }
    }

    fn stop_preview(&self) -> Result<()> {
        capture_engine_stop_preview(&self.engine)?;
        self.wait_for_event(CaptureEngineEvent::PreviewStopped);
        Ok(())
    }

    fn set_media_type(&mut self, media_type: &MediaType) -> Result<()> {
        unsafe {
            let source = self.engine.GetSource()?;
            let sink = self.engine.GetSink(MF_CAPTURE_ENGINE_SINK_TYPE_PREVIEW)?;
            let sink: IMFCapturePreviewSink = sink.cast()?;
            self.engine.StopPreview()?;
            source.SetCurrentDeviceMediaType(0, &media_type.0)?;
            sink.RemoveAllStreams()?;
            let mut rgb_media_type = media_type.clone();
            rgb_media_type.set_rgb32();
            let stream_index = sink.AddStream(0, Some(&media_type.0), None)?;
            let (sample_tx, sample_rx) = channel();
            self.sample_cb = CaptureSampleCallback { sample_tx }.into();
            self.sample_rx = sample_rx;
            sink.SetSampleCallback(stream_index, Some(&self.sample_cb))?;
            self.engine.StartPreview()?;
        }
        self.sample_media_type = Some(media_type.clone());
        Ok(())
    }

    fn wait_for_event(&self, event: CaptureEngineEvent) {
        self.event_rx.iter().find(|e| e == &event);
    }

    fn try_recv_sample(&self) -> Option<LockedBuffer> {
        if let Some(mt) = capture_engine_sink_get_media_type(&self.engine).ok() {
            let width = mt.frame_width();
            let height = mt.frame_height();

            let mut sample = None;
            loop {
                let next_sample = self.sample_rx.try_recv().ok();
                if next_sample.is_some() {
                    sample = next_sample;
                } else {
                    break;
                }
            }

            sample.flatten().and_then(|sample| sample_to_locked_buffer(&sample, width, height).ok())
        } else {
            None
        }
    }

    fn recv_sample(&self) -> Option<LockedBuffer> {
        let Some(mt) = capture_engine_sink_get_media_type(&self.engine).ok() else {
            return None;
        };
        let width = mt.frame_width();
        let height = mt.frame_height();
        self.sample_rx
            .recv()
            .ok()
            .flatten()
            .and_then(|sample| sample_to_locked_buffer(&sample, width, height).ok())
    }
}

fn new_capture_engine() -> Result<IMFCaptureEngine> {
    unsafe {
        let engine_factory: IMFCaptureEngineClassFactory = CoCreateInstance::<Option<&IUnknown>, _>(
            &CLSID_MFCaptureEngineClassFactory,
            None,
            CLSCTX_INPROC_SERVER,
        )?;
        engine_factory.CreateInstance(&CLSID_MFCaptureEngine)
    }
}

fn init_capture_engine(
    capture_engine: &IMFCaptureEngine,
    media_source: Option<&IMFMediaSource>,
    event_cb: &IMFCaptureEngineOnEventCallback,
) -> Result<()> {
    unsafe {
        let video_source =
            if let Some(src) = media_source.cloned() { Some(src.cast()?) } else { None };

        let attributes = mf_create_attributes();
        attributes.SetUINT32(&MF_CAPTURE_ENGINE_USE_VIDEO_DEVICE_ONLY, 1)?;
        capture_engine.Initialize(
            Some(event_cb),
            &attributes,
            None as Option<&IUnknown>,
            video_source.as_ref(),
        )
    }
}

fn capture_engine_start_preview(
    capture_engine: &IMFCaptureEngine,
    sample_cb: &IMFCaptureEngineOnSampleCallback,
) -> Result<MediaType> {
    unsafe {
        let source = capture_engine.GetSource().expect("GetSource");

        // TODO could get video capabilities
        // TODO choose media_type from capabilities and requested format
        let streams = source.GetDeviceStreamCount().expect("GetDeviceStreamCount");
        for index in 0..streams {
            let cat = source.GetDeviceStreamCategory(index).expect("GetDeviceStreamCategory");
            let cat = StreamCategory::from(cat.0);
            println!("{index} {cat:?}");
        }

        let media_type = source.GetCurrentDeviceMediaType(0).expect("GetCurrentDeviceMediaType");
        println!("Source {}", MediaType(media_type.clone()));

        source.SetCurrentDeviceMediaType(0, &media_type).expect("SetCurrentDeviceMediaType");

        let sink = capture_engine.GetSink(MF_CAPTURE_ENGINE_SINK_TYPE_PREVIEW).expect("GetSink");

        let preview_sink: IMFCapturePreviewSink = sink.cast().expect("CapturePreviewSink");

        let mut rgb_media_type = MediaType(media_type);
        rgb_media_type.set_rgb32();

        let stream_index =
            preview_sink.AddStream(0, Some(&rgb_media_type.0), None).expect("AddStream");
        println!("Stream Index {stream_index}");

        preview_sink.SetSampleCallback(stream_index, Some(sample_cb)).expect("SetSampleCallback");

        let output_media_type = MediaType(preview_sink.GetOutputMediaType(stream_index).unwrap());
        println!("Output {output_media_type}");

        capture_engine.StartPreview().expect("StartPreview");

        Ok(output_media_type)
    }
}

fn capture_engine_sink_get_media_type(capture_engine: &IMFCaptureEngine) -> Result<MediaType> {
    Ok(MediaType(unsafe {
        capture_engine.GetSink(MF_CAPTURE_ENGINE_SINK_TYPE_PREVIEW)?.GetOutputMediaType(0)?
    }))
}

fn capture_engine_stop_preview(capture_engine: &IMFCaptureEngine) -> Result<()> {
    unsafe { capture_engine.StopPreview() }
}

fn sample_to_locked_buffer(sample: &IMFSample, width: u32, height: u32) -> Result<LockedBuffer> {
    unsafe {
        let media_buffer = sample.ConvertToContiguousBuffer()?;
        let mf2d_buffer: IMF2DBuffer2 = windows::core::Interface::cast(&media_buffer)?;

        let mut scanline0 = std::ptr::null_mut();
        let mut pitch = 0;
        let mut buffer_start = std::ptr::null_mut();
        let mut buffer_length: u32 = 0;
        mf2d_buffer.Lock2DSize(
            MF2DBuffer_LockFlags_Read,
            &mut scanline0,
            &mut pitch,
            &mut buffer_start,
            &mut buffer_length,
        )?;

        Ok(LockedBuffer {
            buffer: mf2d_buffer,
            width,
            height,
            scanline0,
            len: pitch as usize * height as usize,
        })
    }
}

struct LockedBuffer {
    buffer: IMF2DBuffer2,
    width: u32,
    height: u32,
    scanline0: *mut u8,
    len: usize,
}

impl LockedBuffer {
    fn data(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.scanline0, self.len) }
    }
}

impl Drop for LockedBuffer {
    fn drop(&mut self) {
        unsafe { self.buffer.Unlock2D().expect("Unlock2D") };
    }
}

impl Clone for LockedBuffer {
    fn clone(&self) -> Self {
        unsafe {
            let mut scanline0 = std::ptr::null_mut();
            let mut pitch = 0;
            let mut buffer_start = std::ptr::null_mut();
            let mut buffer_length = 0;
            self.buffer
                .Lock2DSize(
                    MF2DBuffer_LockFlags_Read,
                    &mut scanline0,
                    &mut pitch,
                    &mut buffer_start,
                    &mut buffer_length,
                )
                .unwrap();
        }

        Self {
            buffer: self.buffer.clone(),
            width: self.width.clone(),
            height: self.height.clone(),
            scanline0: self.scanline0.clone(),
            len: self.len.clone(),
        }
    }
}

// // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //

#[allow(unused)]
fn capture_source_collect_available_device_media_types(
    source: &IMFCaptureSource,
) -> Vec<MediaType> {
    (0..)
        .map(|i| unsafe { source.GetAvailableDeviceMediaType(0, i).ok() })
        .take_while(|it| it.is_some())
        .flatten()
        .map(MediaType)
        .collect()
}

impl IMFCaptureEngineOnEventCallback_Impl for CaptureEventCallback {
    fn OnEvent(&self, event: &Option<IMFMediaEvent>) -> windows::core::Result<()> {
        let Some(event) = event else { return Ok(()) };
        let guid = unsafe { event.GetExtendedType().unwrap() };
        let status = unsafe { event.GetStatus().unwrap() };
        let engine_event = CaptureEngineEvent::from(&guid);
        let time = std::time::UNIX_EPOCH.elapsed().unwrap().as_millis();
        println!(
            "Event {engine_event:?} {:x} {} {}",
            status.0,
            status.message().to_string_lossy(),
            time
        );
        self.event_tx.send(engine_event).unwrap();
        Ok(())
    }
}

impl IMFCaptureEngineOnSampleCallback_Impl for CaptureSampleCallback {
    fn OnSample(&self, sample: &core::option::Option<IMFSample>) -> windows::core::Result<()> {
        // if let Some(sample) = sample {
        //     let len = unsafe { sample.GetTotalLength().unwrap() };
        //     let time_us = unsafe { sample.GetSampleTime().unwrap() };
        //     let time_ms = time_us / 10000;
        //     let time = std::time::UNIX_EPOCH.elapsed().unwrap().as_millis();
        //     println!("Sample {len} {time_ms} {time}");
        // };
        self.sample_tx.send(sample.clone()).unwrap();
        Ok(())
    }
}

#[derive(PartialEq, Eq, Debug)]
enum StreamCategory {
    VideoPreview = 0,
    VideoCapture = 1,
    PhotoIndependent = 2,
    PhotoDependent = 3,
    Audio = 4,
    Unsupported = 5,
    Metadata = 6,
}

impl From<i32> for StreamCategory {
    fn from(value: i32) -> Self {
        use StreamCategory::*;
        match value {
            0 => VideoPreview,
            1 => VideoCapture,
            2 => PhotoIndependent,
            3 => PhotoDependent,
            4 => Audio,
            5 => Unsupported,
            6 => Metadata,
            _ => todo!(),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum CaptureEngineEvent {
    Initialized,
    Error,
    PreviewStarted,
    Unknown,
    AllEffectsRemoved,
    CameraStreamBlocked,
    CameraStreamUnblocked,
    EffectAdded,
    EffectRemoved,
    PhotoTaken,
    PreviewStopped,
    RecordStarted,
    RecordStopped,
    SinkPrepared,
    SourceCurrentDeviceMediaTypeSet,
    OutputMediaTypeSet,
}

impl From<&GUID> for CaptureEngineEvent {
    fn from(guid: &GUID) -> Self {
        match *guid {
            MF_CAPTURE_ENGINE_ALL_EFFECTS_REMOVED => CaptureEngineEvent::AllEffectsRemoved,
            MF_CAPTURE_ENGINE_CAMERA_STREAM_BLOCKED => CaptureEngineEvent::CameraStreamBlocked,
            MF_CAPTURE_ENGINE_CAMERA_STREAM_UNBLOCKED => CaptureEngineEvent::CameraStreamUnblocked,
            MF_CAPTURE_ENGINE_EFFECT_ADDED => CaptureEngineEvent::EffectAdded,
            MF_CAPTURE_ENGINE_EFFECT_REMOVED => CaptureEngineEvent::EffectRemoved,
            MF_CAPTURE_ENGINE_ERROR => CaptureEngineEvent::Error,
            MF_CAPTURE_ENGINE_INITIALIZED => CaptureEngineEvent::Initialized,
            MF_CAPTURE_ENGINE_PHOTO_TAKEN => CaptureEngineEvent::PhotoTaken,
            MF_CAPTURE_ENGINE_PREVIEW_STARTED => CaptureEngineEvent::PreviewStarted,
            MF_CAPTURE_ENGINE_PREVIEW_STOPPED => CaptureEngineEvent::PreviewStopped,
            MF_CAPTURE_ENGINE_RECORD_STARTED => CaptureEngineEvent::RecordStarted,
            MF_CAPTURE_ENGINE_RECORD_STOPPED => CaptureEngineEvent::RecordStopped,
            MF_CAPTURE_ENGINE_OUTPUT_MEDIA_TYPE_SET => CaptureEngineEvent::OutputMediaTypeSet,
            MF_CAPTURE_SINK_PREPARED => CaptureEngineEvent::SinkPrepared,
            MF_CAPTURE_SOURCE_CURRENT_DEVICE_MEDIA_TYPE_SET => {
                CaptureEngineEvent::SourceCurrentDeviceMediaTypeSet
            }
            _ => CaptureEngineEvent::Unknown,
        }
    }
}

#[implement(IMFCaptureEngineOnEventCallback)]
struct CaptureEventCallback {
    event_tx: Sender<CaptureEngineEvent>,
}

#[implement(IMFCaptureEngineOnSampleCallback)]
struct CaptureSampleCallback {
    sample_tx: Sender<Option<IMFSample>>,
}

pub fn co_initialize_multithreaded() {
    unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) }.unwrap();
}

pub fn co_uninitialize() {
    unsafe { CoUninitialize() };
}

pub fn co_mta_usage() {
    let _ = unsafe { CoIncrementMTAUsage() };
}
