use std::num::NonZeroU32;

use winit::{
    dpi::PhysicalSize,
    event::{DeviceEvent, ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use kamera::*;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let context = unsafe { softbuffer::Context::new(&window) }.unwrap();
    let mut surface = unsafe { softbuffer::Surface::new(&context, &window) }.unwrap();

    let mut camera = Camera::new_default_device();
    camera.start();

    event_loop.run(move |event, _x, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let Some(frame) = camera.wait_for_frame() else {
                    return
                };
                let (w, h) = frame.size_u32();

                surface.resize(NonZeroU32::new(w).unwrap(), NonZeroU32::new(h).unwrap()).unwrap();
                window.set_inner_size(PhysicalSize::new(w, h));

                let mut buffer = surface.buffer_mut().unwrap();
                let len = buffer.len();
                buffer.copy_from_slice(&frame.data().data_u32()[0..len]);
                buffer.present().unwrap();
            }
            Event::WindowEvent { event: WindowEvent::CloseRequested, window_id }
                if window_id == window.id() =>
            {
                *control_flow = ControlFlow::Exit;
            }
            Event::LoopDestroyed => {
                camera.stop();
            }
            Event::RedrawEventsCleared => {
                window.request_redraw();
            }
            Event::DeviceEvent {
                event: DeviceEvent::Button { button: _, state: ElementState::Released },
                device_id: _,
            } => {
                camera.change_device();
            }
            _ => {}
        }
    });
}
