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
    fn stop_running() {
        AVCaptureSession::new().stop_running();
    }

    #[test]
    fn begin_configuration() {
        AVCaptureSession::new().begin_configuration();
    }

    #[test]
    fn add_input() {
        let device = AVCaptureDevice::default_video_device();
        let input = AVCaptureDeviceInput::from_device(&device).unwrap();
        AVCaptureSession::new().add_input(&*input);
    }

    #[test]
    fn add_output() {
        let output = AVCaptureVideoDataOutput::new();
        AVCaptureSession::new().add_output(&*output);
    }
}

mod av_capture_device_input {
    use super::*;

    #[test]
    fn from_device() {
        let device = AVCaptureDevice::default_video_device();
        let input = AVCaptureDeviceInput::from_device(&device);
        println!("{input:?}");
        assert!(input.is_some());
    }
}

mod av_capture_video_data_output {
    use super::*;

    #[test]
    fn new() {
        let output = AVCaptureVideoDataOutput::new();
        println!("{output:?}");
    }
}

mod scenario {
    use super::*;

    #[test]
    fn running_capture_session() {
        let device = AVCaptureDevice::default_video_device();
        let input = AVCaptureDeviceInput::from_device(&device).unwrap();
        let output = AVCaptureVideoDataOutput::new();
        let delegate = SampleBufferDelegate::new().share();
        let slot = delegate.clone_slot();
        println!("1 {:?}", slot.read().unwrap());
        let session = AVCaptureSession::new();
        output.set_sample_buffer_delegate(delegate.clone());
        println!("2 {:?}", slot.read().unwrap());
        session.add_input(&*input);
        session.add_output(&*output);
        session.start_running();
        std::thread::sleep(std::time::Duration::from_millis(100)); // TODO wait for data
        session.stop_running();
        println!("3 {:?}", slot.read().unwrap());
    }
}
