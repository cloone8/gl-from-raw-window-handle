use raw_gl_context::{GlConfig, GlContext};

use winit::event_loop::{EventLoop};
use winit::window::WindowBuilder;

use winit::event::{Event, WindowEvent};

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let context = unsafe { GlContext::create(&window, GlConfig::default()).unwrap() };

    unsafe {
        context.make_current();
    }

    gl::load_with(|symbol| context.get_proc_address(symbol) as *const _);

    event_loop.run(move |event, window_target| {

        match event {
            Event::WindowEvent {
                event: winit::event::WindowEvent::CloseRequested,
                ..
            } => {
                window_target.exit();
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                unsafe {
                    context.make_current();
                }

                unsafe {
                    gl::ClearColor(0.2, 0.5, 0.8, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }

                context.swap_buffers();

                unsafe {
                    context.make_not_current();
                }
            }
            _ => {}
        }
    }).expect("TODO: panic message");
}
