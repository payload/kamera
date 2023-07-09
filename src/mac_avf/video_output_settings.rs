use icrate::Foundation::*;
use objc2::rc::Id;

pub fn video_settings_from_pixel_format(
    pixel_format: &str,
) -> Id<NSMutableDictionary<NSString, NSNumber>> {
    video_settings_with_pixel_format(str_to_u32(pixel_format))
}

pub fn video_settings_rgb32() -> Id<NSMutableDictionary<NSString, NSNumber>> {
    video_settings_with_pixel_format(32)
}

pub fn video_settings_rgb24() -> Id<NSMutableDictionary<NSString, NSNumber>> {
    video_settings_with_pixel_format(24)
}

fn str_to_u32(string: &str) -> u32 {
    assert_eq!(4, string.len());
    let bytes = string.as_bytes();
    let a = bytes[0];
    let b = bytes[1];
    let c = bytes[2];
    let d = bytes[3];
    let u = unsafe { std::mem::transmute::<[u8; 4], u32>([a, b, c, d]) };
    u
}

fn video_settings_with_pixel_format(
    pixel_format: u32,
) -> Id<NSMutableDictionary<NSString, NSNumber>> {
    let mut settings = NSMutableDictionary::<NSString, NSNumber>::new();
    let px_number = NSNumber::new_u32(pixel_format);
    let px_format_type = NSString::from_str("PixelFormatType"); // kCVPixelBufferPixelFormatTypeKey
    settings.insert(px_format_type, px_number);
    settings
}
