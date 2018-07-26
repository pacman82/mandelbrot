//! Renders a Mandelbrot fractal using OpenGL

#[macro_use]
extern crate glium;

use glium::{glutin, Surface};
use glutin::{dpi::LogicalSize, ControlFlow, WindowEvent};

const VERTEX_SHADER_SRC: &str = include_str!("vertex_shader.glsl");
const PIXEL_SHADER_SRC: &str = include_str!("pixel_shader.glsl");

// The number of iterations used to determine if `c` converges.
const ITERATIONS: i32 = 256;

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_dimensions(LogicalSize::new(500., 500.))
        .with_title("Mandelbrot");
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let shaders =
        glium::Program::from_source(&display, VERTEX_SHADER_SRC, PIXEL_SHADER_SRC, None).unwrap();

    let shape = rect();
    let vertices = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

    let center = (-0.5f32, 0f32);

    draw(&display, &vertices, &indices, &shaders, &center);

    events_loop.run_forever(|event| match event {
        glutin::Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => ControlFlow::Break,
            WindowEvent::Resized(_) | WindowEvent::Refresh => {
                draw(&display, &vertices, &indices, &shaders, &center);
                ControlFlow::Continue
            }
            _ => ControlFlow::Continue,
        },
        _ => ControlFlow::Continue,
    })
}

// A Rectangle covering the whole window
fn rect() -> Vec<Vertex> {
    let v = |x, y| Vertex { position: [x, y] };
    vec![v(-1.0, 1.0), v(-1.0, -1.0), v(1.0, 1.0), v(1.0, -1.0)]
}

fn draw<'i, V, I>(
    display: &glium::Display,
    vertices: &glium::VertexBuffer<V>,
    indices: I,
    shaders: &glium::Program,
    center: &(f32, f32),
) where
    V: Copy,
    I: Into<glium::index::IndicesSource<'i>>,
{
    let draw_parameters = Default::default();

    let mut target = display.draw();
    target
        .draw(
            vertices,
            indices,
            shaders,
            &uniform! { iter: ITERATIONS, center: *center },
            &draw_parameters,
        )
        .unwrap();
    target.finish().unwrap();
}
