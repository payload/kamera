use windows::Win32::Media::MediaFoundation::*;

pub struct VideoFormat(pub windows::core::GUID);

impl std::fmt::Display for VideoFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(mf_video_format_to_str(self.0))
    }
}

// Table of video formats https://learn.microsoft.com/en-us/windows/win32/medfound/video-subtype-guids
#[allow(non_upper_case_globals, unused, non_snake_case)]
fn mf_video_format_to_str(guid: windows::core::GUID) -> &'static str {
    match guid {
        MFVideoFormat_RGB8 => "RGB8",
        MFVideoFormat_RGB555 => "RGB555",
        MFVideoFormat_RGB565 => "RGB565",
        MFVideoFormat_RGB24 => "RGB24",
        MFVideoFormat_RGB32 => "RGB32",
        MFVideoFormat_ARGB32 => "ARGB32",
        MFVideoFormat_A2R10G10B10 => "A2R10G10B10",
        MFVideoFormat_A16B16G16R16F => "A16B16G16R16F",
        MFVideoFormat_AI44 => "AI44",
        MFVideoFormat_AYUV => "AYUV",
        MFVideoFormat_I420 => "I420",
        MFVideoFormat_IYUV => "IYUV",
        MFVideoFormat_NV11 => "NV11",
        MFVideoFormat_NV12 => "NV12",
        MFVideoFormat_NV21 => "NV21",
        MFVideoFormat_UYVY => "UYVY",
        MFVideoFormat_Y41P => "Y41P",
        MFVideoFormat_Y41T => "Y41T",
        MFVideoFormat_Y42T => "Y42T",
        MFVideoFormat_YUY2 => "YUY2",
        MFVideoFormat_YVU9 => "YVU9",
        MFVideoFormat_YV12 => "YV12",
        MFVideoFormat_YVYU => "YVYU",
        MFVideoFormat_P010 => "P010",
        MFVideoFormat_P016 => "P016",
        MFVideoFormat_P210 => "P210",
        MFVideoFormat_P216 => "P216",
        MFVideoFormat_v210 => "v210",
        MFVideoFormat_v216 => "v216",
        MFVideoFormat_v410 => "v410",
        MFVideoFormat_Y210 => "Y210",
        MFVideoFormat_Y216 => "Y216",
        MFVideoFormat_Y410 => "Y410",
        MFVideoFormat_Y416 => "Y416",
        MFVideoFormat_L8 => "L8",
        MFVideoFormat_L16 => "L16",
        MFVideoFormat_D16 => "D16",
        MFVideoFormat_DV25 => "DV25",
        MFVideoFormat_DV50 => "DV50",
        MFVideoFormat_DVC => "DVC",
        MFVideoFormat_DVH1 => "DVH1",
        MFVideoFormat_DVHD => "DVHD",
        MFVideoFormat_DVSD => "DVSD",
        MFVideoFormat_DVSL => "DVSL",
        MFVideoFormat_H263 => "H263",
        MFVideoFormat_H264 => "H264",
        MFVideoFormat_H265 => "H265",
        MFVideoFormat_ES => "ES",
        MFVideoFormat_HEVC => "HEVC",
        MFVideoFormat_ES => "ES",
        MFVideoFormat_M4S2 => "M4S2",
        MFVideoFormat_MJPG => "MJPG",
        MFVideoFormat_MP43 => "MP43",
        MFVideoFormat_MP4S => "MP4S",
        MFVideoFormat_MP4V => "MP4V",
        MFVideoFormat_MPEG2 => "MPEG2",
        MFVideoFormat_VP80 => "VP80",
        MFVideoFormat_VP90 => "VP90",
        MFVideoFormat_MPG1 => "MPG1",
        MFVideoFormat_MSS1 => "MSS1",
        MFVideoFormat_MSS2 => "MSS2",
        MFVideoFormat_WMV1 => "WMV1",
        MFVideoFormat_WMV2 => "WMV2",
        MFVideoFormat_WMV3 => "WMV3",
        MFVideoFormat_WVC1 => "WVC1",
        MFVideoFormat_420O => "420O",
        MFVideoFormat_AV1 => "AV1",
        _ => "Unknown",
    }
}
