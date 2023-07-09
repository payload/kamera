use std::num::NonZeroU32;

use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use kamera::mac_avf::*;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let context = unsafe { softbuffer::Context::new(&window) }.unwrap();
    let mut surface = unsafe { softbuffer::Surface::new(&context, &window) }.unwrap();

    let device = AVCaptureDevice::default_video_device();
    let input = AVCaptureDeviceInput::from_device(&device).unwrap();
    let output = AVCaptureVideoDataOutput::new();
    output.set_video_settings(&video_settings_from_pixel_format("ARGB"));
    let delegate = SampleBufferDelegate::new();
    let slot = delegate.slot();
    let session = AVCaptureSession::new();
    output.set_sample_buffer_delegate(delegate);
    session.add_input(&*input);
    session.add_output(&*output);
    session.start_running();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let sample = slot.wait_for_sample();

                if let Some(sample) = sample {
                    let pixels = sample.pixels();
                    let (w, h) = (pixels.width as u32, pixels.height as u32);

                    surface
                        .resize(NonZeroU32::new(w).unwrap(), NonZeroU32::new(h).unwrap())
                        .unwrap();
                    window.set_inner_size(PhysicalSize::new(w, h));

                    let mut buffer = surface.buffer_mut().unwrap();
                    let len = buffer.len();
                    buffer.copy_from_slice(&pixels.u32[..len]);
                    buffer.present().unwrap();
                }

                window.request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                *control_flow = ControlFlow::Exit;
            }
            Event::LoopDestroyed => {
                session.stop_running();
            }
            _ => {}
        }
    });
}
