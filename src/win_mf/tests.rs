use super::mf::*;

#[test]
fn device_enum_devices() {
    co_mta_usage();
    assert!(!Device::enum_devices().is_empty());
}

fn first_device() -> Device {
    Device::enum_devices()[0].clone()
}

#[test]
fn device_name() {
    co_mta_usage();
    let name = first_device().name();
    println!("{name}");
    assert!(!name.is_empty());
}

#[test]
fn device_query_media_types() {
    co_mta_usage();
    let types: Vec<String> =
        first_device().query_media_types().into_iter().map(|d| d.to_string()).collect();
    println!("{types:?}");
    assert!(!types.is_empty());
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
    assert!(!types.is_empty());
}
