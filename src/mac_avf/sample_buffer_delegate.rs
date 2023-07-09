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

use super::{CMSampleBuffer, CMSampleBufferRef};

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
            println!("on_output_sample_buffer {:?}", sample_buffer);
        }

        #[method(captureOutput:didDropSampleBuffer:fromConnection:)]
        unsafe fn on_drop_sample_buffer(
            &self,
            _capture_output: *const c_void,
            _sample_buffer: CMSampleBufferRef,
            _connection: *const c_void,
        ) {
            println!("on_drop_sample_buffer")
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

    pub fn wait_for_sample(&self) {
        let mut _guard = self.state.lock().unwrap();
        _guard = self.condvar.wait(_guard).unwrap();
    }

    fn set_sample(&self, sample: CMSampleBufferRef) {
        self.sample
            .store(sample, std::sync::atomic::Ordering::Relaxed);
    }

    fn notify_all(&self) {
        self.condvar.notify_all();
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
