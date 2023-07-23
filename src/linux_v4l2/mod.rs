use ffimage::color::{Bgra, Rgba};
use v4l;
use v4l::io::traits::CaptureStream;
use v4l::prelude::*;
use v4l::video::capture::Parameters;
use v4l::video::Capture;
use v4l::*;

use std::{
    sync::{mpsc::*, RwLock},
    thread,
};

use crate::{InnerCamera, InnerFrame, InnerFrameData};

pub struct Camera {
    device: RwLock<v4l::Device>,
    device_name: String,
    stream: RwLock<Option<v4l::io::mmap::Stream<'static>>>,
}

impl InnerCamera for Camera {
    type Frame = Frame;

    fn new_default_device() -> Self {
        let device_node = v4l::context::enum_devices().into_iter().next().unwrap();
        let device_name =
            device_node.name().unwrap_or_else(|| device_node.path().to_string_lossy().to_string());

        println!(
            "Node {{ index: {}, name: {:?}, path: {:?} }}",
            device_node.index(),
            device_node.name(),
            device_node.path()
        );

        let mut device = v4l::Device::new(0).unwrap();

        for fmt in device.enum_formats().unwrap() {
            println!("{:?}", fmt);

            for size in device.enum_framesizes(fmt.fourcc).unwrap() {
                println!("{:?}", size);
            }
        }

        let rgb = FourCC::new(b"RGB3");
        let mut fmt = device.format().unwrap();
        let size = device
            .enum_framesizes(fmt.fourcc)
            .unwrap()
            .into_iter()
            .next()
            .unwrap()
            .size
            .to_discrete()
            .into_iter()
            .last()
            .unwrap();
        fmt.width = size.width;
        fmt.height = size.height;

        if let Err(error) = device.set_format(&fmt) {
            eprintln!("Device.set_format: {}", error);
        }

        Self { device: RwLock::new(device), device_name, stream: RwLock::new(None) }
    }

    fn start(&self) {
        if self.stream.read().unwrap().is_none() {
            let device = self.device.write().unwrap();
            let stream =
                v4l::io::mmap::Stream::with_buffers(&device, v4l::buffer::Type::VideoCapture, 4)
                    .expect("Failed to create buffer stream");
            let _ = self.stream.write().unwrap().insert(stream);
        }
    }

    fn stop(&self) {
        let _ = self.stream.write().unwrap().take();
    }

    fn wait_for_frame(&self) -> Option<Frame> {
        let format = self.device.read().unwrap().format().unwrap();
        let size = (format.width, format.height);
        if let Ok((buf, _meta)) = self.stream.write().unwrap().as_mut().unwrap().next() {
            let data = match &format.fourcc.repr {
                b"RGB3" => buf.to_vec(),
                b"YUYV" => yuyv_to_rgb32(buf, size.0, size.1),
                b"MJPG" => todo!("NJPG not implemented"),
                _ => panic!("invalid buffer pixelformat"),
            };

            Some(Frame { data, size })
        } else {
            None
        }
    }
}

impl std::fmt::Debug for Camera {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Camera").field("device", &self.device_name).finish()
    }
}

pub struct Frame {
    data: Vec<u8>,
    size: (u32, u32),
}

impl InnerFrame for Frame {
    type FrameData = FrameData;

    fn data(&self) -> FrameData {
        FrameData { data: self.data.clone() }
    }

    fn size_u32(&self) -> (u32, u32) {
        self.size
    }
}

impl std::fmt::Debug for Frame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Frame").field("data", &self.data.len()).finish()
    }
}

#[derive(Debug)]
pub struct FrameData {
    data: Vec<u8>,
}

impl InnerFrameData for FrameData {
    fn data_u8(&self) -> &[u8] {
        &self.data
    }

    fn data_u32(&self) -> &[u32] {
        unsafe { self.data.align_to().1 }
    }
}

fn yuyv_to_rgb32(buf: &[u8], w: u32, h: u32) -> Vec<u8> {
    use ffimage::color::Rgb;
    use ffimage::packed::{ImageBuffer, ImageView};
    use ffimage::traits::Convert;
    use ffimage_yuv::{yuv::Yuv, yuyv::Yuyv};

    let yuv422 = ImageView::<Yuyv<u8>>::from_buf(buf, w, h).unwrap();
    let mut yuv444 = ImageBuffer::<Yuv<u8>>::new(w, h, 0u8);
    let mut rgb = ImageBuffer::<Rgb<u8>>::new(w, h, 0u8);
    let mut rgba = ImageBuffer::<Bgra<u8>>::new(w, h, 0u8);
    yuv422.convert(&mut yuv444);
    yuv444.convert(&mut rgb);
    rgb.convert(&mut rgba);

    rgba.into_buf()
}
