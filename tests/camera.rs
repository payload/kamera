use kamera::Camera;

#[test]
fn new_default_device() {
    Camera::new_default_device();
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
    let (w, h) = frame.size_u32();
    let data1 = frame.data();
    let data2 = frame.data();
    println!("data len {}", data1.data_u32().len());
    println!("data len {}", data2.data_u32().len());
    println!("data len {}", data1.data_u32().len());
    let a = data1.data_u32();
    let b = data2.data_u32();
    assert_eq!(a, b);
}
