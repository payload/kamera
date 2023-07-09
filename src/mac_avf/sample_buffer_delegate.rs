use std::ffi::c_void;
use std::ptr::null_mut;
use std::sync::atomic::AtomicPtr;
use std::sync::{Arc, Condvar, Mutex};

use icrate::Foundation::NSObjectProtocol;
use objc2::{
    declare::{Ivar, IvarDrop},
    mutability::Mutable,
    rc::Id,
    runtime::NSObject,
    *,
};

use super::{CMSampleBuffer, CMSampleBufferRef, SampleBuffer};

declare_class!(
    pub struct SampleBufferDelegate {
        slot: IvarDrop<Box<Arc<Slot>>, "_slot">,
    }

    mod ivars;

    unsafe impl ClassType for SampleBufferDelegate {
        type Super = NSObject;
        type Mutability = Mutable;
        const NAME: &'static str = "SampleBufferDelegate";
    }

    unsafe impl SampleBufferDelegate {
        #[method(captureOutput:didOutputSampleBuffer:fromConnection:)]
        unsafe fn on_output_sample_buffer(
            &mut self,
            _capture_output: *const c_void,
            sample_buffer: CMSampleBufferRef,
            _connection: *const c_void,
        ) {
            self.set_slot(sample_buffer);
        }

        #[method(captureOutput:didDropSampleBuffer:fromConnection:)]
        unsafe fn on_drop_sample_buffer(
            &mut self,
            _capture_output: *const c_void,
            sample_buffer: CMSampleBufferRef,
            _connection: *const c_void,
        ) {
            println!("DROP SAMPLE BUFFER UNIMPLEMENTED");
            self.set_slot(sample_buffer);
        }
    }

    unsafe impl NSObjectProtocol for SampleBufferDelegate {}
);

impl SampleBufferDelegate {
    pub fn new() -> Id<Self> {
        let mut this: Id<Self> = unsafe { msg_send_id![Self::class(), new] };
        Ivar::write(&mut this.slot, Box::new(Arc::new(Slot::new())));
        this
    }

    pub fn slot(&self) -> Arc<Slot> {
        (**self.slot).clone()
    }

    fn set_slot(&mut self, sample: CMSampleBufferRef) {
        self.slot.set_sample(sample);
        self.slot.notify_all();
    }
}

#[derive(Debug)]
pub struct Slot {
    sample: AtomicPtr<CMSampleBuffer>,
    state: Mutex<State>,
    condvar: Condvar,
}

impl Slot {
    fn new() -> Self {
        Self {
            sample: AtomicPtr::new(null_mut()),
            state: Mutex::new(State { frame_counter: 0 }),
            condvar: Condvar::new(),
        }
    }

    pub fn wait_for_sample(&self) -> Option<SampleBuffer> {
        let mut _guard = self.state.lock().unwrap();
        _guard = self.condvar.wait(_guard).unwrap();
        let ptr = self.sample.load(std::sync::atomic::Ordering::Relaxed);
        if ptr.is_null() {
            None
        } else {
            Some(SampleBuffer::new(ptr))
        }
    }

    fn set_sample(&self, mut sample: CMSampleBufferRef) {
        // TODO should instead use SampleBuffer directly, it already wraps Retain and Release
        sample = if !sample.is_null() {
            unsafe { super::CFRetain(sample.cast()).cast_mut().cast() }
        } else {
            sample
        };
        let old_sample = self.sample.swap(sample, std::sync::atomic::Ordering::Relaxed);
        if !old_sample.is_null() {
            unsafe { super::CFRelease(old_sample.cast()) };
        }
    }

    fn notify_all(&self) {
        self.condvar.notify_all();
    }
}

impl Drop for Slot {
    fn drop(&mut self) {
        let sample = self.sample.load(std::sync::atomic::Ordering::Relaxed);
        if !sample.is_null() {
            unsafe { super::CFRelease(sample.cast()) };
        }
    }
}

#[derive(Debug, Clone)]
pub struct State {
    pub frame_counter: usize,
}

#[test]
fn msg_send_to_on_output_sample_buffer() {
    use std::ptr::null;
    let delegate = SampleBufferDelegate::new();
    let output: *const c_void = null();
    let buffer: CMSampleBufferRef = null_mut();
    let connection: *const c_void = null();
    let () = unsafe {
        msg_send![&delegate, captureOutput: output didOutputSampleBuffer: buffer fromConnection: connection]
    };
}

#[test]
fn msg_send_to_on_drop_sample_buffer() {
    use std::ptr::null;
    let delegate = SampleBufferDelegate::new();
    let output: *const c_void = null();
    let buffer: CMSampleBufferRef = null_mut();
    let connection: *const c_void = null();
    let () = unsafe {
        msg_send![&delegate, captureOutput: output didDropSampleBuffer: buffer fromConnection: connection]
    };
}

#[test]
fn slot() {
    let delegate = SampleBufferDelegate::new();
    println!("slot {:?}", delegate.slot);
}
