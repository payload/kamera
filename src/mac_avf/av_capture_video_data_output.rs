use std::{ffi::*, ptr::null};

use objc::*;
use objc_foundation::*;
use objc_id::*;

use super::SampleBufferDelegate;

object_struct!(AVCaptureVideoDataOutput);
impl IAVCaptureVideoDataOutput for AVCaptureVideoDataOutput {}
impl IAVCaptureOutput for AVCaptureVideoDataOutput {}

pub trait IAVCaptureVideoDataOutput: IAVCaptureOutput {
    fn set_sample_buffer_delegate(&self, delegate: Id<SampleBufferDelegate, Shared>) {
        let name = std::ffi::CString::new("video input").unwrap();
        let queue = unsafe { dispatch_queue_create(name.as_ptr(), null()) };
        unsafe { msg_send!(self, setSampleBufferDelegate: delegate queue: queue) }
    }
}

pub trait IAVCaptureOutput: INSObject {}

// libdispatch is loaded differently on MacOS and iOS. Have a look in https://docs.rs/dispatch
// We don't care about the exact types.
#[link(name = "System", kind = "dylib")]
extern "C" {
    pub fn dispatch_queue_create(name: *const c_char, attr: *const c_void) -> DispatchQueueT;
    pub fn dispatch_release(queue: DispatchQueueT);
}

pub type DispatchQueueT = *mut NSObject;
