use std::{ffi::OsString, os::windows::prelude::OsStringExt};

use windows::Win32::Media::MediaFoundation::*;
use windows::Win32::System::Com::CoTaskMemFree;

pub fn mf_create_attributes() -> IMFAttributes {
    let mut attributes: Option<IMFAttributes> = None;
    unsafe { MFCreateAttributes(&mut attributes, 1) }.unwrap();
    attributes.unwrap()
}

pub fn mf_get_string(
    activate: &IMFActivate,
    guid: &windows::core::GUID,
) -> windows::core::Result<OsString> {
    let mut buf = windows::core::PWSTR::null();
    let mut len = 0;
    unsafe { activate.GetAllocatedString(guid, &mut buf, &mut len) }?;
    let str = OsString::from_wide(unsafe { buf.as_wide() });
    unsafe { CoTaskMemFree(Some(buf.as_ptr() as *const _)) };
    Ok(str)
}
