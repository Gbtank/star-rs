use std::rc::Rc;

use glium::*;
use glium::uniforms::{SamplerBehavior, MagnifySamplerFilter, MinifySamplerFilter};
use imgui::*;

use glium::{implement_vertex, backend::Facade};


use imgui_glium_renderer::{Renderer, Texture};


mod app;
use app::{App, State};
mod shader;
use crate::shader::Shader;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

fn draw<F>(gl_ctx: &F, renderer: &mut Renderer, state: &mut State) -> TextureId where F: Facade {
    // Create an empty texture and turn it into a simple frame buffer which can be drawn on.
    let tex = Texture2d::empty(gl_ctx, 512, 512).unwrap();
    let mut fbo = tex.as_surface();

    // Set up rectangular scene for fragment shader.
    let vertex1 = Vertex { position: [-1.0, -1.0] };
    let vertex2 = Vertex { position: [-1.0,  1.0] };
    let vertex3 = Vertex { position: [ 1.0, -1.0] };
    let vertex4 = Vertex { position: [ 1.0,  1.0] };
    let shape = vec![vertex1, vertex2, vertex3, vertex4];
    let order: Vec<u8> = vec![0, 1, 2, 1, 2, 3];

    let vshader = Shader::from("glsl/vertex.glsl");
    let fshader = Shader::from("glsl/fragment.glsl");

    let vertex_buffer = glium::VertexBuffer::new(gl_ctx, &shape).unwrap();
    let indices = glium::IndexBuffer::new(gl_ctx, glium::index::PrimitiveType::TrianglesList, &order).unwrap();

    let program = glium::Program::from_source(gl_ctx, &vshader.src, &fshader.src, None).unwrap();    
    
    let uniforms = uniform! {
        lum:  state.luminosity,
        layers: state.layers as i32,
        seed: state.seed,
        res: [512.0f32, 512.0f32],
    };

    fbo.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();
    let texture = Texture {
        texture: Rc::new(tex),
        sampler: SamplerBehavior {
            magnify_filter: MagnifySamplerFilter::Linear,
            minify_filter: MinifySamplerFilter::Linear,
            ..Default::default()
        },
    };
    renderer.textures().insert(texture)
}

use std::time::{Duration, SystemTime, };
fn main() {
    // Pseudo-random seed when initialising program.
    let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or(Duration::ZERO);

    // Initial shader state, which is changed by user with fields.
    let state = app::State {
        luminosity: 0.2,
        layers: 3,
        seed: time.as_secs() as i32,
    };

    let mut app = App::init(state);
    let mut texture_id = draw(&app.display, &mut app.renderer, &mut app.state);

    app.main_loop(move |display, renderer, state, ui| {
            // Settings Window
            Window::new("Settings").build(&ui, || {
                Slider::new("Luminosity", 0.0, 1.0).build(&ui, &mut state.luminosity);
                Slider::new("Layers", 1, 10).build(&ui, &mut state.layers);
                ui.input_int("Seed", &mut state.seed).build();
                if ui.button("Generate") {
                    texture_id = draw(display, renderer, state);
                }
            });

            // Image Window
            Window::new("Image").size([533.0, 548.0], Condition::Always).build(&ui, || {
                Image::new(texture_id, [512.0, 512.0]).build(&ui);
            });
    });
}