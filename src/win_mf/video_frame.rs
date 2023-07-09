use windows::Win32::Media::MediaFoundation::*;

#[derive(Debug, PartialEq, Eq)]
pub struct VideoFrame {
    pub width: u32,
    pub height: u32,
    pub sample: IMFSample,
}

// TODO IMFSample is not Send or Sync but I unsafe impl Send here
//
// The Lock2D method is not a thread synchronisation method but ensures
// that once Lock2D succeeds you indeed have a valid buffer pointer in your
// hands until Unlock is called. Media Foundation will reuse IMFSamples when possible.
// At least this is what can be observed when looking at the debug prints of the pointer in
// the sample. It is repeating.
//
// Anyway I think that it is valid to lock the buffer readonly, move it around across threads
// as you please and unlock on Drop. VideoFrame nore Pixels is a proper wrapper for this yet.
unsafe impl Send for VideoFrame {}

impl VideoFrame {
    pub fn pixels(&self) -> Pixels {
        let sample = &self.sample;
        let media_buffer = unsafe { sample.ConvertToContiguousBuffer() }.unwrap();
        let mf2d_buffer: IMF2DBuffer2 = windows::core::Interface::cast(&media_buffer).unwrap();

        let mut scanline0 = std::ptr::null_mut();
        let mut pitch = 0;
        let mut buffer_start = std::ptr::null_mut();
        let mut buffer_length = 0;
        unsafe {
            mf2d_buffer.Lock2DSize(
                MF2DBuffer_LockFlags_Read,
                &mut scanline0,
                &mut pitch,
                &mut buffer_start,
                &mut buffer_length,
            )
        }
        .unwrap();
        let data =
            unsafe { std::slice::from_raw_parts(scanline0, pitch as usize * self.height as usize) };

        Pixels { data, mf2d_buffer }
    }
}

pub struct Pixels<'a> {
    pub data: &'a [u8],
    mf2d_buffer: IMF2DBuffer2,
}

impl<'a> Drop for Pixels<'a> {
    fn drop(&mut self) {
        unsafe { self.mf2d_buffer.Unlock2D() }.unwrap();
    }
}
