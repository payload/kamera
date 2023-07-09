use icrate::Foundation::NSObjectProtocol;
use objc2::runtime::NSObject;
use objc2::{extern_class, mutability, ClassType};

extern_class!(
    #[derive(PartialEq, Eq, Hash, Debug)]
    pub struct AVCaptureDeviceFormat;

    unsafe impl ClassType for AVCaptureDeviceFormat {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for AVCaptureDeviceFormat {}
