use std::ffi::c_void;

use objc2::{Encode, Encoding, RefEncode};

pub struct SampleBuffer {
    inner: CMSampleBufferRef,
}

impl SampleBuffer {
    pub fn new(sample_buffer: CMSampleBufferRef) -> Self {
        Self { inner: unsafe { CFRetain(sample_buffer.cast()).cast_mut().cast() } }
    }

    pub fn size_usize(&self) -> (usize, usize) {
        let ibuf = unsafe { CMSampleBufferGetImageBuffer(self.inner) };
        let width = unsafe { CVPixelBufferGetWidth(ibuf) };
        let height = unsafe { CVPixelBufferGetHeight(ibuf) };
        (width, height)
    }
}

impl Drop for SampleBuffer {
    fn drop(&mut self) {
        unsafe { CFRelease(self.inner.cast()) };
    }
}

impl SampleBuffer {
    pub fn pixels(&self) -> Pixels {
        Pixels::new(self)
    }
}

impl std::fmt::Debug for SampleBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sbuf = self.inner;
        let format = unsafe { CMSampleBufferGetFormatDescription(sbuf) };
        let dim = unsafe { CMVideoFormatDescriptionGetDimensions(format) };
        let fourcc = unsafe { CMFormatDescriptionGetMediaSubType(format) };
        let pixel_format = fourcc_to_string(fourcc);
        let width = dim.width;
        let height = dim.height;
        let name = format!("SampleBuffer {width}x{height} {pixel_format:?}");

        f.debug_struct(&name).field("inner", &self.inner).finish()
    }
}

#[link(name = "CoreMedia", kind = "framework")]
extern "C" {
    pub fn CMSampleBufferGetFormatDescription(sbuf: CMSampleBufferRef) -> CMFormatDescriptionRef;
    pub fn CMSampleBufferGetImageBuffer(sbuf: CMSampleBufferRef) -> CVImageBufferRef;
    pub fn CMFormatDescriptionGetMediaSubType(desc: CMFormatDescriptionRef) -> u32;
    pub fn CMVideoFormatDescriptionGetDimensions(desc: CMFormatDescriptionRef)
        -> CMVideoDimensions;
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    pub fn CFRetain(cf: *const c_void) -> *const c_void;
    pub fn CFRelease(cf: *const c_void);
}

#[link(name = "CoreVideo", kind = "framework")]
extern "C" {
    pub fn CVPixelBufferLockBaseAddress(buf: CVBufferRef, option: u64) -> i32;
    pub fn CVPixelBufferUnlockBaseAddress(buf: CVBufferRef, option: u64) -> i32;
    pub fn CVPixelBufferGetBaseAddress(buf: CVBufferRef) -> *const u8;
    pub fn CVPixelBufferGetBytesPerRow(buf: CVBufferRef) -> usize;
    pub fn CVPixelBufferGetWidth(buf: CVBufferRef) -> usize;
    pub fn CVPixelBufferGetHeight(buf: CVBufferRef) -> usize;
    pub fn CVPixelBufferIsPlanar(buf: CVBufferRef) -> bool;
    pub fn CVPixelBufferGetPlaneCount(buf: CVBufferRef) -> usize;
    pub fn CVPixelBufferGetHeightOfPlane(buf: CVBufferRef, index: usize) -> usize;
    pub fn CVPixelBufferGetBytesPerRowOfPlane(buf: CVBufferRef, index: usize) -> usize;
    pub fn CVPixelBufferGetDataSize(buf: CVBufferRef) -> usize;
    pub fn CVPixelBufferGetPixelFormatType(buf: CVBufferRef) -> u32;
    pub fn CVPixelBufferGetBaseAddressOfPlane(buf: CVBufferRef, index: usize) -> *const u8;
}

#[repr(C)]
pub struct CVBuffer {
    _priv: [u8; 0],
}
pub type CVBufferRef = *const CVBuffer;
pub type CVImageBufferRef = CVBufferRef;

#[repr(C)]
#[derive(Debug)]
pub struct CMVideoDimensions {
    pub width: i32,
    pub height: i32,
}

#[repr(C)]
pub struct CMSampleBuffer {
    _priv: [u8; 0],
}
pub type CMSampleBufferRef = *mut CMSampleBuffer;

unsafe impl Encode for CMSampleBuffer {
    const ENCODING: Encoding = Encoding::Struct("opaqueCMSampleBuffer", &[]);
}
unsafe impl RefEncode for CMSampleBuffer {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(C)]
pub struct CMFormatDescription {
    _priv: [u8; 0],
}
pub type CMFormatDescriptionRef = *mut CMFormatDescription;

/// FOURCC is a little crazy. Look at some references to interpret this obfuscation.
/// Look also into Chromium. There you can see that NV12 is a preferred format, 420v on Mac.
///
/// Also note that 420v means "video range" (420f means "full range") and this means a reduced
/// range for Y [16, 235] and UV [16, 240] (ITU-R BT 601).
/// And even full range would be Y [0, 255] and UV [1, 255].
///
/// <https://chromium.googlesource.com/libyuv/libyuv/+/HEAD/docs/formats.md>
/// <https://softron.zendesk.com/hc/en-us/articles/207695697-List-of-FourCC-codes-for-video-codecs>
/// <http://abcavi.kibi.ru/fourcc.php>
pub fn fourcc_to_string(px_format_u32: u32) -> String {
    let bytes = px_format_u32.to_ne_bytes();

    if &bytes[1..4] == &[0, 0, 0] {
        match bytes[0] {
            32 => "ARGB",
            24 => "RGB ",
            _ => return format!("0x{px_format_u32:08X}"),
        }
        .into()
    } else {
        String::from_utf8_lossy(&bytes).to_string()
    }
}

/// Holds the locked pixel data of a frame and unlocks upon drop.
pub struct Pixels<'a> {
    pub ibuf: CVImageBufferRef,
    pub data: &'a [u8],
    pub u32: &'a [u32],
    pub width: usize,
    pub height: usize,
}

impl<'a> Pixels<'a> {
    fn new(sample: &'a SampleBuffer) -> Self {
        let ibuf = unsafe { CMSampleBufferGetImageBuffer(sample.inner) };
        debug_assert!(0 == unsafe { CVPixelBufferLockBaseAddress(ibuf, 1) });
        let _address = unsafe { CVPixelBufferGetBaseAddress(ibuf) };
        let stride = unsafe { CVPixelBufferGetBytesPerRow(ibuf) };
        let width = unsafe { CVPixelBufferGetWidth(ibuf) };
        let height = unsafe { CVPixelBufferGetHeight(ibuf) };
        let is_planar = unsafe { CVPixelBufferIsPlanar(ibuf) };
        let plane_count = unsafe { CVPixelBufferGetPlaneCount(ibuf) };
        let _data_size = unsafe { CVPixelBufferGetDataSize(ibuf) };
        let _fourcc = unsafe { CVPixelBufferGetPixelFormatType(ibuf) };
        let plane_address = unsafe { CVPixelBufferGetBaseAddressOfPlane(ibuf, 0) };
        let mut plane_sizes = 0;

        // println!("pixels {:?}", (_address, stride, width, height, is_planar, plane_count, _data_size, fourcc_to_string(_fourcc)));
        if is_planar {
            for index in 0..plane_count {
                let _plane_address = unsafe { CVPixelBufferGetBaseAddressOfPlane(ibuf, index) };
                let plane_stride = unsafe { CVPixelBufferGetBytesPerRowOfPlane(ibuf, index) };
                let plane_height = unsafe { CVPixelBufferGetHeightOfPlane(ibuf, index) };
                // println!("        {:?}", (plane_address, plane_stride, plane_height));
                plane_sizes += plane_stride * plane_height;
            }
        } else {
            plane_sizes += stride * height;
        }

        let data = unsafe { std::slice::from_raw_parts(plane_address, plane_sizes) };
        let u32 = unsafe { std::slice::from_raw_parts(plane_address as *const u32, plane_sizes) };
        Self { ibuf, data, u32, width, height }
    }
}

impl Drop for Pixels<'_> {
    fn drop(&mut self) {
        debug_assert!(0 == unsafe { CVPixelBufferUnlockBaseAddress(self.ibuf, 1) });
    }
}
