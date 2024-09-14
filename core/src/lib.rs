pub(crate) mod app_logic;
pub(crate) mod boilerplate;

use app_logic::AppState;
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event_loop::{ControlFlow::Wait, EventLoop},
    window::{Window, WindowAttributes},
};

pub fn run() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}

enum RenderState {
    Active(AppState),
    Suspended(Option<Arc<Window>>),
}

impl Default for RenderState {
    fn default() -> Self {
        Self::Suspended(None)
    }
}

#[derive(Default)]
struct App {
    render_state: RenderState,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = if let RenderState::Suspended(Some(window)) = &self.render_state {
            Arc::clone(window)
        } else {
            let window = event_loop
                .create_window(WindowAttributes::default())
                .unwrap();
            Arc::new(window)
        };

        self.render_state = RenderState::Active(AppState::new(window));
    }

    fn suspended(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if let RenderState::Active(app_state) = &self.render_state {
            self.render_state = RenderState::Suspended(Some(app_state.window()));
        }
        event_loop.set_control_flow(Wait);
    }

    fn about_to_wait(&mut self, _: &winit::event_loop::ActiveEventLoop) {
        let RenderState::Active(app_state) = &mut self.render_state else {
            return;
        };
        app_state.window().request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let RenderState::Active(app_state) = &mut self.render_state else {
            return;
        };
        match event {
            winit::event::WindowEvent::CloseRequested => event_loop.exit(),
            winit::event::WindowEvent::Resized(size) => {
                app_state.resize(size);
            }
            winit::event::WindowEvent::RedrawRequested => {
                app_state.draw().unwrap();
            }
            _ => (),
        }
    }
}
