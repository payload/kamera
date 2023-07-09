use std::ffi::c_void;
use std::sync::Once;

use objc::declare::ClassDecl;
use objc::runtime::*;
use objc::*;
use objc_foundation::*;

use crate::mac_avf::AVCaptureVideoDataOutput;

pub enum SampleBufferDelegate {}
unsafe impl Message for SampleBufferDelegate {}

trait AVCaptureVideoDataOutputSampleBufferDelegate {
    // captureOutput:didOutputSampleBuffer:fromConnection:
    fn on_output_sample_buffer(
        &self,
        capture_output: *const c_void,
        sample_buffer: *const c_void,
        connection: *const c_void,
    );

    fn on_drop_sample_buffer(&self, capture_output: (), sample_buffer: (), connection: ());
}

impl AVCaptureVideoDataOutputSampleBufferDelegate for SampleBufferDelegate {
    fn on_output_sample_buffer(
        &self,
        capture_output: *const c_void,
        sample_buffer: *const c_void,
        connection: *const c_void,
    ) {
        println!(
            "OUTPUT {:?} {:?} {:?} {:?}",
            self as *const _, capture_output, sample_buffer, connection
        );
    }

    fn on_drop_sample_buffer(&self, capture_output: (), sample_buffer: (), connection: ()) {
        todo!()
    }
}

impl SampleBufferDelegate {
    fn number(&self) -> u32 {
        unsafe {
            let obj = &*(self as *const _ as *const Object);
            *obj.get_ivar("_number")
        }
    }

    fn set_number(&mut self, number: u32) {
        unsafe {
            let obj = &mut *(self as *mut _ as *mut Object);
            obj.set_ivar("_number", number);
        }
    }
}

static REGISTER_CLASS: Once = Once::new();

impl INSObject for SampleBufferDelegate {
    fn class() -> &'static Class {
        REGISTER_CLASS.call_once(|| {
            let superclass = NSObject::class();
            let mut decl = ClassDecl::new("SampleBufferDelegate", superclass).unwrap();

            extern "C" fn on_output_sample_buffer(
                this: &mut Object,
                _cmd: Sel,
                capture_output: *const c_void,
                sample_buffer: *const c_void,
                connection: *const c_void,
            ) {
                let that: *const SampleBufferDelegate = unsafe { std::mem::transmute(this) };
                let that = unsafe { that.as_ref().unwrap() };
                SampleBufferDelegate::on_output_sample_buffer(
                    that,
                    capture_output,
                    sample_buffer,
                    connection,
                )
            }

            unsafe {
                decl.add_method(
                    sel!(captureOutput:didOutputSampleBuffer:fromConnection:),
                    on_output_sample_buffer as extern "C" fn(&mut Object, Sel, _, _, _),
                )
            };

            extern "C" fn on_drop_sample_buffer(
                this: &mut Object,
                _cmd: Sel,
                capture_output: *const c_void,
                sample_buffer: *const c_void,
                connection: *const c_void,
            ) {
                println!("DROP {:?}", this as *const Object);
            }

            unsafe {
                decl.add_method(
                    sel!(captureOutput:didDropSampleBuffer:fromConnection:),
                    on_drop_sample_buffer as extern "C" fn(&mut Object, Sel, _, _, _),
                )
            };

            decl.register();
        });

        Class::get("SampleBufferDelegate").unwrap()
    }
}

#[test]

fn main() {
    SampleBufferDelegate::new();
}
