use kamera::mac_avf::*;

mod av_capture_device {
    use super::*;

    #[test]
    fn default_video_device() {
        let device = AVCaptureDevice::default_video_device();
        println!("{device:#?}");
    }

    #[test]
    fn all_video_devices() {
        let devices = AVCaptureDevice::all_video_devices();
        println!("{:#?}", devices.to_vec());
        assert!(devices.count() > 0);
    }

    #[test]
    fn unique_id() {
        for device in AVCaptureDevice::all_video_devices().to_vec() {
            println!("{}", device.unique_id().as_str());
            assert!(device.unique_id().len() > 0);
        }
    }

    #[test]
    fn localized_name() {
        for device in AVCaptureDevice::all_video_devices().to_vec() {
            println!("{}", device.localized_name().as_str());
            assert!(device.localized_name().len() > 0);
        }
    }

    #[test]
    fn formats() {
        for device in AVCaptureDevice::all_video_devices().to_vec() {
            println!("{:#?}", device.formats().to_vec());
            assert!(device.formats().count() > 0);
        }
    }
}

mod av_capture_session {
    use super::*;

    #[test]
    fn new() {
        println!("{:?}", AVCaptureSession::new());
    }

    #[test]
    fn start_running() {
        AVCaptureSession::new().start_running();
    }

    #[test]
    fn begin_configuration() {
        AVCaptureSession::new().begin_configuration();
    }
}

mod av_capture_device_input {
    use super::*;

    #[test]
    fn from_device() {
        let device = AVCaptureDevice::default_video_device();
        let input = AVCaptureDeviceInput::from_device(&device);
        assert!(input.is_some());
    }
}
