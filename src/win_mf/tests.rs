use super::win_mf::*;

#[test]
fn device_enum_devices() {
    co_mta_usage();
    assert!(Device::enum_devices().len() > 0);
}

fn first_device() -> Device {
    Device::enum_devices()[0].clone()
}

#[test]
fn device_name() {
    co_mta_usage();
    let name = first_device().name();
    println!("{name}");
    assert!(name.len() > 0);
}

#[test]
fn device_query_media_types() {
    co_mta_usage();
    let types: Vec<String> =
        first_device().query_media_types().into_iter().map(|d| d.to_string()).collect();
    println!("{types:?}");
    assert!(types.len() > 0);
}

#[test]
fn device_query_media_types_with_best_fps() {
    co_mta_usage();
    let types: Vec<String> = first_device()
        .query_media_types_with_best_fps()
        .into_iter()
        .map(|d| d.to_string())
        .collect();
    println!("{types:?}");
    assert!(types.len() > 0);
}

#[test]
fn get_single_frame() {
    co_mta_usage();
    let mut camera: Camera = Camera::new();
    camera.set_device(&first_device());
    camera.start();
    assert!(camera.wait_frame().is_some());
    camera.stop();
}

#[test]
fn camera_device_name() {
    co_mta_usage();
    let camera = Camera::from_device(&first_device());
    let name = camera.device_name();
    assert!(!name.is_empty());
    println!("{name}");
}

#[test]
fn start_stop_start() {
    co_mta_usage();
    let mut camera: Camera = Camera::new();
    camera.set_device(&first_device());
    camera.start();
    assert!(camera.wait_frame().is_some());
    camera.stop();
    camera.just_start();
    assert!(camera.wait_frame().is_some());
    camera.stop();
}

#[test]
fn camera_device_name_twice() {
    co_mta_usage();
    let camera = Camera::from_device(&first_device());
    let name = camera.device_name();
    assert!(!name.is_empty());
    println!("{name}");

    let camera = Camera::from_device(&first_device());
    let name = camera.device_name();
    assert!(!name.is_empty());
    println!("{name}");
}
