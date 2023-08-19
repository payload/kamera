use super::*;

use objc2::runtime::AnyClass;

fn reflect_class(cls: &AnyClass) -> Result<String, std::fmt::Error> {
    use std::fmt::Write;
    let mut reflection = String::new();
    let s = &mut reflection;

    for var in cls.instance_variables().iter() {
        let name = var.name();
        let var_type = var.type_encoding();
        writeln!(s, "var {name}: {var_type:?}")?;
    }

    for method in cls.instance_methods().iter() {
        let name = method.name().to_string();
        let ret = method.return_type();
        let args = (0..method.arguments_count())
            .filter_map(|i| method.argument_type(i))
            .collect::<Vec<_>>();
        let simple_method = args.len() == 2 && args[0].eq("@") && args[1].eq(":"); // [Object, Sel]
        let known_ret = encoding_to_rust_typename(&ret);

        let ret = if !known_ret.is_empty() { known_ret.into() } else { ret.to_string() };

        let args = if simple_method { "&self".into() } else { format!("{args:?}") };

        writeln!(s, "fn {name}({args}) -> {ret}")?;
    }

    Ok(reflection)
}

fn encoding_to_rust_typename(enc: &str) -> &'static str {
    match enc {
        "c" => "i8",
        "s" => "i16",
        "i" => "i32",
        "q" => "i64",
        "C" => "u8",
        "S" => "u16",
        "I" => "u32",
        "Q" => "u64",
        "f" => "f32",
        "d" => "f64",
        "B" => "bool",
        "v" => "()",
        "*" => "*mut c_char",
        "r*" => "*const c_char",
        "^v" => "*mut c_void",
        "r^v" => "*const c_void",
        ":" => "Sel",
        "@" => "NSObject",
        _ => "",
    }
}

#[test]
fn av_capture_device() {
    println!("{}", reflect_class(AVCaptureDevice::class()).unwrap());
}

#[test]
fn av_capture_device_format() {
    println!("{}", reflect_class(AVCaptureDeviceFormat::class()).unwrap());
}

#[test]
fn av_capture_session() {
    println!("{}", reflect_class(AVCaptureSession::class()).unwrap());
}

#[test]
fn av_capture_device_input() {
    println!("{}", reflect_class(AVCaptureDeviceInput::class()).unwrap());
}
