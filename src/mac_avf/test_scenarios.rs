use super::*;

const TEST_FRAMES: usize = 3;

#[test]
fn running_capture_session() {
    let device = AVCaptureDevice::default_video_device();
    let input = AVCaptureDeviceInput::from_device(&device).unwrap();
    let output = AVCaptureVideoDataOutput::new();
    let delegate = SampleBufferDelegate::new();
    let slot = delegate.slot();
    let session = AVCaptureSession::new();
    output.set_sample_buffer_delegate(delegate);
    session.add_input(&input);
    session.add_output(&output);
    session.start_running();

    std::iter::from_fn(|| slot.wait_for_sample())
        .map(|s| println!("{s:?}"))
        .take(TEST_FRAMES)
        .count();

    session.stop_running();
}

#[test]
fn running_capture_session_for_all_cameras() {
    println!();
    for device in AVCaptureDevice::all_video_devices() {
        println!("{}", device.localized_name());
        let input = AVCaptureDeviceInput::from_device(&device).unwrap();
        let output = AVCaptureVideoDataOutput::new();
        let delegate = SampleBufferDelegate::new();
        let slot = delegate.slot();
        let session = AVCaptureSession::new();
        output.set_sample_buffer_delegate(delegate);
        session.add_input(&input);
        session.add_output(&output);
        session.start_running();

        std::iter::from_fn(|| slot.wait_for_sample())
            .map(|s| println!("{s:?}"))
            .take(TEST_FRAMES)
            .count();

        session.stop_running();
    }
}

#[test]
fn running_capture_session_for_all_cameras_in_rgb32() {
    println!();
    for device in AVCaptureDevice::all_video_devices() {
        println!("{}", device.localized_name());
        let input = AVCaptureDeviceInput::from_device(&device).unwrap();
        let output = AVCaptureVideoDataOutput::new();
        output.set_video_settings(&video_settings_rgb32());
        let delegate = SampleBufferDelegate::new();
        let slot = delegate.slot();
        let session = AVCaptureSession::new();
        output.set_sample_buffer_delegate(delegate);
        session.add_input(&input);
        session.add_output(&output);
        session.start_running();

        std::iter::from_fn(|| slot.wait_for_sample())
            .map(|s| println!("{s:?}"))
            .take(TEST_FRAMES)
            .count();

        session.stop_running();
    }
}

#[test]
fn running_capture_session_for_all_cameras_in_rgb24() {
    println!();
    for device in AVCaptureDevice::all_video_devices() {
        println!("{}", device.localized_name());
        let input = AVCaptureDeviceInput::from_device(&device).unwrap();
        let output = AVCaptureVideoDataOutput::new();
        output.set_video_settings(&video_settings_rgb24());
        let delegate = SampleBufferDelegate::new();
        let slot = delegate.slot();
        let session = AVCaptureSession::new();
        output.set_sample_buffer_delegate(delegate);
        session.add_input(&input);
        session.add_output(&output);
        session.start_running();

        std::iter::from_fn(|| slot.wait_for_sample())
            .map(|s| println!("{s:?}"))
            .take(TEST_FRAMES)
            .count();

        session.stop_running();
    }
}

#[test]
fn running_capture_session_for_all_cameras_in_yuv2() {
    println!();
    for device in AVCaptureDevice::all_video_devices() {
        println!("{}", device.localized_name());
        let input = AVCaptureDeviceInput::from_device(&device).unwrap();
        let output = AVCaptureVideoDataOutput::new();
        output.set_video_settings(&video_settings_from_pixel_format("yuv2"));
        let delegate = SampleBufferDelegate::new();
        let slot = delegate.slot();
        let session = AVCaptureSession::new();
        output.set_sample_buffer_delegate(delegate);
        session.add_input(&input);
        session.add_output(&output);
        session.start_running();

        std::iter::from_fn(|| slot.wait_for_sample())
            .map(|s| println!("{s:?}"))
            .take(TEST_FRAMES)
            .count();

        session.stop_running();
    }
}
