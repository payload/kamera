use windows::Win32::Media::MediaFoundation::*;

#[derive(Debug, PartialEq, Eq)]
pub enum MFSourceReaderFlag {
    Sample,
    Error,
    EndOfStream,
    NewStream,
    NativeMediaTypeChanged,
    CurrentMediaTypeChanged,
    StreamTick,
    AllEffectsRemoved,
    Unknown(u32),
}

impl From<u32> for MFSourceReaderFlag {
    fn from(value: u32) -> Self {
        use MFSourceReaderFlag::*;
        let flag = MF_SOURCE_READER_FLAG(value as i32);
        match flag {
            MF_SOURCE_READER_FLAG(0) => Sample,
            MF_SOURCE_READERF_ERROR => Error,
            MF_SOURCE_READERF_ENDOFSTREAM => EndOfStream,
            MF_SOURCE_READERF_NEWSTREAM => NewStream,
            MF_SOURCE_READERF_NATIVEMEDIATYPECHANGED => NativeMediaTypeChanged,
            MF_SOURCE_READERF_CURRENTMEDIATYPECHANGED => CurrentMediaTypeChanged,
            MF_SOURCE_READERF_STREAMTICK => StreamTick,
            MF_SOURCE_READERF_ALLEFFECTSREMOVED => AllEffectsRemoved,
            _ => Unknown(value),
        }
    }
}
