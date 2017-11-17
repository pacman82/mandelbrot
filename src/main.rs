#[macro_use]
extern crate glium;

use glium::{glutin, Surface};

const VERTEX_SHADER_SRC: &str = include_str!("vertex_shader.glsl");
const PIXEL_SHADER_SRC: &str = include_str!("pixel_shader.glsl");

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new().with_title("Mandelbrot");
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let shaders =
        glium::Program::from_source(&display, VERTEX_SHADER_SRC, PIXEL_SHADER_SRC, None).unwrap();

    // draw mandelbrot
    let shape = rect();
    let vertices = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

    draw(&display, &vertices, &indices, &shaders);

    draw(&display, &vertices, &indices, &shaders);

    events_loop.run_forever(|event| match event {
        glutin::Event::WindowEvent { event, .. } => match event {
            glutin::WindowEvent::Closed => glutin::ControlFlow::Break,
            glutin::WindowEvent::Resized(_, _) => {
                draw(&display, &vertices, &indices, &shaders);
                glutin::ControlFlow::Continue
            }
            _ => glutin::ControlFlow::Continue,
        },
        _ => glutin::ControlFlow::Continue,
    })
}

// A Rectangle covering the whole window
fn rect() -> Vec<Vertex> {
    vec![
        Vertex {
            position: [-1.0, 1.0],
        },
        Vertex {
            position: [-1.0, -1.0],
        },
        Vertex {
            position: [1.0, 1.0],
        },
        Vertex {
            position: [1.0, -1.0],
        },
    ]
}

fn draw<'i, V, I>(
    display: &glium::Display,
    vertices: &glium::VertexBuffer<V>,
    indices: I,
    shaders: &glium::Program,
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
            &glium::uniforms::EmptyUniforms,
            &draw_parameters,
        )
        .unwrap();
    target.finish().unwrap();
}
