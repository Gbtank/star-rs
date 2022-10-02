use std::time::Instant;

use glium::{Display, glutin::{self, event::{Event, WindowEvent}, event_loop::ControlFlow}, Surface};
use imgui::{Context, Ui};
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{WinitPlatform, HiDpiMode};

pub struct State {
    pub luminosity: f32,
    pub layers: u8,
    pub seed: i32,
}

pub struct App {
    pub event_loop: glutin::event_loop::EventLoop<()>,
    pub display: glium::Display,
    pub imgui: Context,
    pub platform: WinitPlatform,
    pub renderer: Renderer,
    pub state: State,
}

impl App {
    pub fn init(state: State) -> Self {
        let event_loop = glutin::event_loop::EventLoop::new();
        let wb = glutin::window::WindowBuilder::new().with_title("Star-rs Renderer");
        let cb = glutin::ContextBuilder::new();
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    
        let mut imgui = imgui::Context::create();
    
        let mut platform = WinitPlatform::init(&mut imgui);
        {
            let gl_window = display.gl_window();
            let window = gl_window.window();
    
            let dpi_mode = if let Ok(factor) = std::env::var("IMGUI_EXAMPLE_FORCE_DPI_FACTOR") {
                // Allow forcing of HiDPI factor for debugging purposes
                match factor.parse::<f64>() {
                    Ok(f) => HiDpiMode::Locked(f),
                    Err(e) => panic!("Invalid scaling factor: {}", e),
                }
            } else {
                HiDpiMode::Default
            };
    
            platform.attach_window(imgui.io_mut(), window, dpi_mode);
        }
    
        let renderer = Renderer::init(&mut imgui, &display).unwrap();

        App {
            event_loop,
            display,
            imgui,
            platform,
            renderer,
            state,
        }
    }

    pub fn main_loop<F: FnMut(&Display, &mut Renderer, &mut State, &Ui) + 'static>(self, mut run_ui: F) {
        let App {
            event_loop,
            display,
            mut imgui,
            mut platform,
            mut renderer,
            mut state,
        } = self;

        let mut last_frame = Instant::now();

        event_loop.run(move |event, _, control_flow| match event {
            Event::NewEvents(_) => {
                let now = Instant::now();
                imgui.io_mut().update_delta_time(now - last_frame);
                last_frame = now;
            }
            Event::MainEventsCleared => {
                let gl_window = display.gl_window();
                platform
                    .prepare_frame(imgui.io_mut(), gl_window.window())
                    .expect("Failed to prepare frame");
                gl_window.window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                let ui = imgui.frame();
    
                run_ui(&display, &mut renderer, &mut state, &ui);
    
                let gl_window = display.gl_window();
                let mut target = display.draw();
                target.clear_color_srgb(0.1, 0.1, 0.1, 1.0);
                platform.prepare_render(&ui, gl_window.window());
                let draw_data = ui.render();
                renderer
                    .render(&mut target, draw_data)
                    .expect("Rendering failed");
                target.finish().expect("Failed to swap buffers");
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            event => {
                let gl_window = display.gl_window();
                platform.handle_event(imgui.io_mut(), gl_window.window(), &event);
            }
        });
    }
}