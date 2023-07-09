use kamera::mac_avf::*;

#[test]
fn running_capture_session() {
    let device = AVCaptureDevice::default_video_device();
    let input = AVCaptureDeviceInput::from_device(&device).unwrap();
    let output = AVCaptureVideoDataOutput::new();
    let delegate = SampleBufferDelegate::new();
    let slot = delegate.slot();
    let session = AVCaptureSession::new();
    output.set_sample_buffer_delegate(delegate);
    session.add_input(&*input);
    session.add_output(&*output);
    session.start_running();

    slot.wait_for_sample();
    slot.wait_for_sample();
    slot.wait_for_sample();

    session.stop_running();
}
