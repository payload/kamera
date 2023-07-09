use kamera::mac_avf::*;

#[test]
fn running_capture_session() {
    let device = AVCaptureDevice::default_video_device();
    let input = AVCaptureDeviceInput::from_device(&device).unwrap();
    let output = AVCaptureVideoDataOutput::new();
    // let delegate = SampleBufferDelegate::new().share();
    // let slot = delegate.clone_slot();
    // let (lock, cond) = &*slot;
    let session = AVCaptureSession::new();
    // output.set_sample_buffer_delegate(delegate.clone());
    session.add_input(&*input);
    session.add_output(&*output);
    session.start_running();

    // {
    //     let mut guard = lock.lock().unwrap();
    //     while guard.frame_counter < 3 {
    //         guard = cond.wait(guard).unwrap();
    //     }
    // }
    std::thread::sleep(std::time::Duration::from_millis(100));

    session.stop_running();
}
