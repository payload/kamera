use std::ffi::c_void;
use std::ptr::{null, null_mut};
use std::sync::atomic::AtomicPtr;
use std::sync::{Arc, Condvar, Mutex, Once};

use icrate::Foundation::NSObjectProtocol;
use objc2::mutability::{InteriorMutable, Mutable};
use objc2::{rc::Id, runtime::NSObject, *};

use icrate::ns_string;
use objc2::{
    declare::{Ivar, IvarDrop},
    declare_class, extern_methods, msg_send,
    rc::Allocated,
    runtime::{Object, ProtocolObject, Sel},
    sel, ClassType,
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
        #[method(initWith:)]
        unsafe fn __initWith(&mut self, arg: u32) -> Option<&mut Self> {
            let this: Option<&mut Self> = msg_send![super(self), init];
            let this = this?;
            Ivar::write(&mut this.slot, Box::new(Arc::new(Slot::new())));
            Some(this)
        }

        #[method(captureOutput:didOutputSampleBuffer:fromConnection:)]
        unsafe fn on_output_sample_buffer(
            &mut self,
            capture_output: *const c_void,
            sample_buffer: CMSampleBufferRef,
            connection: *const c_void,
        ) {
            self.set_slot(sample_buffer);
            println!("on_output_sample_buffer {:?}", sample_buffer);
        }

        #[method(captureOutput:didDropSampleBuffer:fromConnection:)]
        unsafe fn on_drop_sample_buffer(
            &self,
            capture_output: *const c_void,
            sample_buffer: CMSampleBufferRef,
            connection: *const c_void,
        ) {
            println!("on_drop_sample_buffer")
        }
    }

    unsafe impl NSObjectProtocol for SampleBufferDelegate {}
);

impl SampleBufferDelegate {
    pub fn new() -> Id<Self> {
        unsafe { msg_send_id![Self::alloc(), initWith: 5u32] }
    }

    pub fn slot(&self) -> Arc<Slot> {
        (**self.slot).clone()
    }

    fn set_slot(&mut self, sample: CMSampleBufferRef) {
        self.slot.set_sample(sample);
        self.slot.notify_all();
    }
}

impl Drop for SampleBufferDelegate {
    fn drop(&mut self) {
        println!("SampleBufferDelegate Drop");
    }
}

extern_methods!(
    unsafe impl SampleBufferDelegate {
        #[method_id(initWith:)]
        #[allow(non_snake_case)]
        pub fn initWith(this: Option<Allocated<Self>>, fun: u32) -> Id<Self>;
    }
);

#[test]
fn msg_send_to_on_output_sample_buffer() {
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

#[derive(Debug)]
pub struct Fun(u32);

impl Drop for Fun {
    fn drop(&mut self) {
        println!("Fun Drop {}", self.0);
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
        let mut guard = self.state.lock().unwrap();
        guard = self.condvar.wait(guard).unwrap();
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
