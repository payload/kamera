use windows::Win32::Media::MediaFoundation::*;

use super::VideoFormat;

#[derive(Debug, Clone)]
pub struct MediaType(pub IMFMediaType);

impl std::fmt::Display for MediaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let frame_format = unsafe { self.0.GetGUID(&MF_MT_SUBTYPE) }.unwrap();
        let video_format = VideoFormat(frame_format);
        let (frame_rate_num, frame_rate_denum) = self.frame_rate();
        let (frame_width, frame_height) = self.frame_size();

        f.write_fmt(format_args!(
            "{frame_width}x{frame_height}@{frame_rate_num}/{frame_rate_denum}({video_format})"
        ))
    }
}

impl MediaType {
    pub fn frame_size(&self) -> (u32, u32) {
        unsafe { self.0.GetUINT64(&MF_MT_FRAME_SIZE) }.map(MediaType::unpack_u64).unwrap_or((0, 0))
    }

    pub fn frame_rate(&self) -> (u32, u32) {
        unsafe { self.0.GetUINT64(&MF_MT_FRAME_RATE) }.map(MediaType::unpack_u64).unwrap_or((0, 1))
    }

    pub fn set_rgb32(&mut self) {
        unsafe { self.0.SetGUID(&MF_MT_SUBTYPE, &MFVideoFormat_RGB32) }.unwrap();
    }

    pub fn frame_rate_f32(&self) -> f32 {
        let (n, d) = self.frame_rate();
        n as f32 / d as f32
    }

    fn unpack_u64(v: u64) -> (u32, u32) {
        ((v >> 32) as _, (v << 32 >> 32) as _)
    }

    pub fn frame_width(&self) -> u32 {
        self.frame_size().0
    }

    pub fn frame_height(&self) -> u32 {
        self.frame_size().1
    }

    pub fn filter_resolutions_with_max_fps(media_types: &[MediaType]) -> Vec<MediaType> {
        let mut resolutions = std::collections::HashMap::<(u32, u32), MediaType>::new();
        for mt in media_types.iter() {
            let res = mt.frame_size();

            if let Some(other) = resolutions.get(&res) {
                if other.frame_rate_f32() >= mt.frame_rate_f32() {
                    continue;
                }
            }

            resolutions.insert(res, mt.clone());
        }
        let mut media_types: Vec<_> = resolutions.into_values().collect();
        media_types.sort_by_key(|mt| mt.frame_size());
        media_types
    }
}

impl PartialEq for MediaType {
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.0.Compare(&other.0, MF_ATTRIBUTES_MATCH_ALL_ITEMS) }.unwrap().as_bool()
    }
}
