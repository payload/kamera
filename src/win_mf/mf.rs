use std::{ffi::OsString, mem::MaybeUninit, sync::mpsc::*};

use windows::{
    core::*,
    Win32::{Media::MediaFoundation::*, System::Com::*},
};

use super::attributes::{mf_create_attributes, mf_get_string};
use super::media_type::MediaType;

#[derive(Clone, Debug)]
pub struct Device {
    pub activate: IMFActivate,
    pub source: IMFMediaSource,
}

impl Device {
    pub(crate) fn new(activate: IMFActivate) -> Self {
        co_initialize_multithreaded();
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
    }
}

impl std::fmt::Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Device({})", self.name()))
    }
}

#[allow(unused)]
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

pub(crate) fn enum_device_sources() -> Vec<IMFActivate> {
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

pub(crate) fn media_foundation_startup() -> Result<()> {
    unsafe { MFStartup(MF_API_VERSION, MFSTARTUP_NOSOCKET) }
}

#[allow(unused)]
pub(crate) fn media_foundation_shutdown() -> Result<()> {
    unsafe { MFShutdown() }
}

// TODO use and fix it
pub(crate) fn _capture_engine_change_media_type(
    engine: &IMFCaptureEngine,
    media_type: &MediaType,
) -> Result<()> {
    unsafe {
        let source = engine.GetSource()?;
        let sink = engine.GetSink(MF_CAPTURE_ENGINE_SINK_TYPE_PREVIEW)?;
        let sink: IMFCapturePreviewSink = sink.cast()?;
        engine.StopPreview()?;

        source.SetCurrentDeviceMediaType(0, &media_type.0)?;
        sink.RemoveAllStreams()?;
        let mut rgb_media_type = media_type.clone();
        rgb_media_type.set_rgb32();
        let stream_index = sink.AddStream(0, Some(&media_type.0), None)?;

        // TODO maybe changing the sample callback is not necessary when the stream_index is the same?
        let (sample_tx, _sample_rx) = channel();
        let sample_cb = CaptureSampleCallback { sample_tx }.into();
        sink.SetSampleCallback(stream_index, Some(&sample_cb))?;

        engine.StartPreview()?;
    }
    Ok(())
}

pub(crate) fn new_capture_engine() -> Result<IMFCaptureEngine> {
    unsafe {
        let engine_factory: IMFCaptureEngineClassFactory = CoCreateInstance::<Option<&IUnknown>, _>(
            &CLSID_MFCaptureEngineClassFactory,
            None,
            CLSCTX_INPROC_SERVER,
        )?;
        engine_factory.CreateInstance(&CLSID_MFCaptureEngine)
    }
}

pub(crate) fn init_capture_engine(
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

pub(crate) fn capture_engine_prepare_sample_callback(
    capture_engine: &IMFCaptureEngine,
    sample_cb: &IMFCaptureEngineOnSampleCallback,
) -> Result<()> {
    unsafe {
        let source = capture_engine.GetSource().expect("GetSource");
        let media_type = source.GetCurrentDeviceMediaType(0).expect("GetCurrentDeviceMediaType");
        let sink = capture_engine.GetSink(MF_CAPTURE_ENGINE_SINK_TYPE_PREVIEW).expect("GetSink");
        let preview_sink: IMFCapturePreviewSink = sink.cast().expect("CapturePreviewSink");
        let mut rgb_media_type = MediaType(media_type);
        rgb_media_type.set_rgb32();
        let stream_index =
            preview_sink.AddStream(0, Some(&rgb_media_type.0), None).expect("AddStream");
        // let stream_index = preview_sink.AddStream(0, None, None).expect("AddStream");

        preview_sink.SetSampleCallback(stream_index, Some(sample_cb)).expect("SetSampleCallback");
    }
    Ok(())
}

pub fn capture_engine_sink_get_media_type(capture_engine: &IMFCaptureEngine) -> Result<MediaType> {
    Ok(MediaType(unsafe {
        capture_engine.GetSink(MF_CAPTURE_ENGINE_SINK_TYPE_PREVIEW)?.GetOutputMediaType(0)?
    }))
}

pub(crate) fn capture_engine_stop_preview(capture_engine: &IMFCaptureEngine) -> Result<()> {
    unsafe { capture_engine.StopPreview() }
}

pub fn sample_to_locked_buffer(
    sample: &IMFSample,
    width: u32,
    height: u32,
) -> Result<LockedBuffer> {
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

#[derive(Debug)]
pub struct LockedBuffer {
    buffer: IMF2DBuffer2,
    pub(crate) width: u32,
    pub(crate) height: u32,
    scanline0: *mut u8,
    len: usize,
}

impl LockedBuffer {
    pub(crate) fn data(&self) -> &[u8] {
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
            width: self.width,
            height: self.height,
            scanline0: self.scanline0,
            len: self.len,
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
        let _ = self.event_tx.send(engine_event);
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
pub(crate) enum CaptureEngineEvent {
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
pub(crate) struct CaptureEventCallback {
    pub event_tx: Sender<CaptureEngineEvent>,
}

#[implement(IMFCaptureEngineOnSampleCallback)]
pub(crate) struct CaptureSampleCallback {
    pub sample_tx: Sender<Option<IMFSample>>,
}

pub fn co_initialize_multithreaded() {
    if let Err(err) = unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) } {
        if err.code() == HRESULT(0x80010106u32 as i32) {
            // "Cannot change thread mode after it is set."
            // Ignore this error and hope for the best until we know better how to deal with this case.
        } else {
            panic!("{err}");
        }
    }
}

// TODO when to use this?
// pub fn co_uninitialize() {
//     unsafe { CoUninitialize() };
// }

#[cfg(test)]
pub fn co_mta_usage() {
    let _ = unsafe { CoIncrementMTAUsage() };
}
