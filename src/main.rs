use render::RenderState;
use window::Window;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

mod render;
mod window;

fn main() {
    pollster::block_on(run_app());
}

async fn run_app() {
    let event = EventLoop::new();
    let window = Window::new().builder().build(&event).unwrap();
    let mut render_state = RenderState::new(&window).await;

    event.run(move |event, _, control_flow| {
        control_flow.set_poll();

        match event {
            Event::WindowEvent {
                window_id,
                ref event,
            } if window_id == window.id() => {
                if !render_state.input(event) {
                    match event {
                        WindowEvent::CloseRequested => control_flow.set_exit(),
                        WindowEvent::Resized(size) => render_state.resize(*size),
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            render_state.resize(**new_inner_size)
                        }
                        _ => (),
                    }
                }
            }
            Event::RedrawRequested(window_id) => {
                if window_id == window.id() {
                    render_state.update();
                    match render_state.render() {
                        Ok(_) => {}
                        // Reconfigure the surface if lost
                        Err(wgpu::SurfaceError::Lost) => render_state.resize(render_state.size),
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SurfaceError::OutOfMemory) => control_flow.set_exit(),
                        // All other errors (Outdated, Timeout) should be resolved by the next frame
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => (),
        }
    })
}
