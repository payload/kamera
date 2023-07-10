use kamera::Camera;

#[test]
fn new_default_device() {
    let camera = Camera::new_default_device();
    println!("{:?}", camera);
}

#[test]
fn start() {
    let camera = Camera::new_default_device();
    camera.start();
}

#[test]
fn start_stop() {
    let camera = Camera::new_default_device();
    camera.start();
    camera.stop();
}

#[test]
fn stop_without_start() {
    let camera = Camera::new_default_device();
    camera.stop();
}

#[test]
fn start_and_wait_for_frames() {
    let camera = Camera::new_default_device();
    camera.start();
    assert!(camera.wait_for_frame().is_some());
    assert!(camera.wait_for_frame().is_some());
    assert!(camera.wait_for_frame().is_some());
    println!("{:?}", camera.wait_for_frame());
}

#[test]
fn excessive_start_calls() {
    let camera = Camera::new_default_device();
    camera.start();
    camera.start();
    assert!(camera.wait_for_frame().is_some());
    camera.start();
    assert!(camera.wait_for_frame().is_some());
    assert!(camera.wait_for_frame().is_some());
    camera.start();
    camera.start();
    println!("{:?}", camera.wait_for_frame());
}

#[test]
fn frame_size() {
    let camera = Camera::new_default_device();
    camera.start();
    let frame = camera.wait_for_frame().unwrap();
    println!("{:?}", frame.size_u32());
    assert!(frame.size_u32().0 > 0 && frame.size_u32().1 > 0);
}

#[test]
fn frame_data() {
    let camera = Camera::new_default_device();
    camera.start();
    let frame = camera.wait_for_frame().unwrap();
    let (_w, _h) = frame.size_u32();
    let data1 = frame.data();
    let data2 = frame.data();
    println!("data len {}", data1.data_u32().len());
    println!("data len {}", data2.data_u32().len());
    println!("data len {}", data1.data_u32().len());
    let a = data1.data_u32();
    let b = data2.data_u32();
    assert_eq!(a, b);
}

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
// linux_v4l2: ioctl VIDIOC_REQBUFS fails with Device Busy, Chromium also fails in this case, no alternative on this level
// win_mf: fails to get frames because "The video recording device is preempted by another immersice application"
#[test]
fn two_cameras_start_and_wait_for_frames() {
    let camera1 = Camera::new_default_device();
    camera1.start();
    println!("Camera 1 {:?}", camera1.wait_for_frame());
    assert!(camera1.wait_for_frame().is_some());
    let camera2 = Camera::new_default_device();
    camera2.start();
    println!("Camera 2 {:?}", camera2.wait_for_frame());
    assert!(camera2.wait_for_frame().is_some());
    assert!(camera1.wait_for_frame().is_some());
    println!("Camera 1 {:?}", camera1.wait_for_frame());
    println!("Camera 2 {:?}", camera2.wait_for_frame());
}

#[test]
fn change_device() {
    let mut camera = Camera::new_default_device();
    camera.start();
    assert!(camera.wait_for_frame().is_some());
    assert!(camera.wait_for_frame().is_some());
    assert!(camera.wait_for_frame().is_some());
    camera.change_device();
    assert!(camera.wait_for_frame().is_some());
    assert!(camera.wait_for_frame().is_some());
    assert!(camera.wait_for_frame().is_some());
}
