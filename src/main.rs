mod renderer;

use std::sync::{Arc, Mutex, RwLock};
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use crate::renderer::graphics_renderer::GraphicsRenderer;

fn handle_event(event: &Event<()>, window: &winit::window::Window, renderer: &mut GraphicsRenderer, control_flow: &mut ControlFlow) {
    match event {
        Event::MainEventsCleared => window.request_redraw(),
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == &window.id() => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input:
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Escape),
                    ..
                },
                ..
            } => *control_flow = ControlFlow::Exit,
            WindowEvent::Resized(physical_size) => {
                renderer.resize(*physical_size);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                renderer.resize(**new_inner_size);
            }
            _ => {}
        },
        _ => {}
    }
}

fn update(dt: instant::Duration, renderer: &mut GraphicsRenderer) {
    tracy_client::Client::running().unwrap().span(tracy_client::span_location!("update"), 0);
    // state.update(&renderer.queue, dt);
}

fn render(control_flow: &mut ControlFlow, renderer: &mut GraphicsRenderer) {
    tracy_client::Client::running().unwrap().span(tracy_client::span_location!("render"), 0);
    match renderer.render_frame(|view, command| {
        // default_state.render(view, command)
        Ok(())
    }) {
        Ok(_) => {}
        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
            renderer.resize(renderer.size)
        }
        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
        Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
    }
    tracy_client::Client::running().unwrap().frame_mark();
}

pub async fn run () {
    env_logger::init();

    tracy_client::Client::start();
    tracy_client::Client::running().unwrap().set_thread_name("MAIN THREAD");

    let event_loop = EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_title("Voxel Test")
        .build(&event_loop)
        .unwrap();

    let renderer = Arc::from(Mutex::from(GraphicsRenderer::initialize(&window).await));
    let mut last_render_time = instant::Instant::now();

    event_loop.run(move |base_event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        let mut renderer_guard = renderer.lock().unwrap();
        handle_event(&base_event, &window, &mut renderer_guard, control_flow);
        let now = instant::Instant::now();
        let dt = now - last_render_time;
        last_render_time = now;
        update(dt, &mut renderer_guard);
        render(control_flow, &mut renderer_guard);
    });
}

fn main() {
    async_std::task::block_on(run());
}