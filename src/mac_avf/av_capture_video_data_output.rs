use std::ffi::*;

use icrate::Foundation::NSObjectProtocol;
use objc2::rc::Id;
use objc2::runtime::NSObject;
use objc2::{extern_class, msg_send_id, mutability, ClassType};

// use super::SampleBufferDelegate;

extern_class!(
    #[derive(PartialEq, Eq, Hash, Debug)]
    pub struct AVCaptureVideoDataOutput;

    unsafe impl ClassType for AVCaptureVideoDataOutput {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for AVCaptureVideoDataOutput {}

// object_struct!(AVCaptureVideoDataOutput);
// impl IAVCaptureVideoDataOutput for AVCaptureVideoDataOutput {}
// impl IAVCaptureOutput for AVCaptureVideoDataOutput {}

impl AVCaptureVideoDataOutput {
    pub fn new() -> Id<Self> {
        unsafe { msg_send_id![Self::class(), new] }
    }

    // pub fn set_sample_buffer_delegate(&self, delegate: Id<SampleBufferDelegate, Shared>) {
    //     let name = std::ffi::CString::new("video input").unwrap();
    //     let queue = unsafe { dispatch_queue_create(name.as_ptr(), null()) };
    //     unsafe { msg_send!(self, setSampleBufferDelegate: delegate queue: queue) }
    // }
}

// libdispatch is loaded differently on MacOS and iOS. Have a look in https://docs.rs/dispatch
// We don't care about the exact types.
#[link(name = "System", kind = "dylib")]
extern "C" {
    pub fn dispatch_queue_create(name: *const c_char, attr: *const c_void) -> DispatchQueueT;
    pub fn dispatch_release(queue: DispatchQueueT);
}

pub type DispatchQueueT = *mut NSObject;

#[test]
fn new() {
    let output = AVCaptureVideoDataOutput::new();
    println!("{output:?}");
}
