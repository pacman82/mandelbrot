//! Renders a Mandelbrot fractal using OpenGL

use glium::{
    glutin,
    glutin::{dpi::LogicalSize, ControlFlow, ElementState, VirtualKeyCode, WindowEvent},
    implement_vertex, uniform, Surface,
};
use std::cmp::{max, min};

const VERTEX_SHADER_SRC: &str = include_str!("vertex_shader.glsl");
const MANDELBROT_PIXEL_SHADER_SRC: &str = include_str!("mandelbrot.glsl");
const JULIA_PIXEL_SHADER_SRC: &str = include_str!("julia.glsl");

// The number of iterations used to determine if `c` converges.
const ITERATIONS: i32 = 256;

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

fn main() {
    let mut iterations = ITERATIONS;
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_dimensions(LogicalSize::new(500., 500.))
        .with_title("Mandelbrot");
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mandlebrot_shaders =
        glium::Program::from_source(&display, VERTEX_SHADER_SRC, MANDELBROT_PIXEL_SHADER_SRC, None).unwrap();
    let julia_shaders =
        glium::Program::from_source(&display, VERTEX_SHADER_SRC, JULIA_PIXEL_SHADER_SRC, None).unwrap();

    let mut current_shaders = &mandlebrot_shaders;

    let shape = rect();
    let vertices = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

    let mut cam = Camera::new();

    draw(&display, &vertices, &indices, current_shaders, iterations, cam.inv_view());

    events_loop.run_forever(|event| match event {
        glutin::Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => ControlFlow::Break,
            WindowEvent::Resized(_) | WindowEvent::Refresh => {
                draw(&display, &vertices, &indices, current_shaders, iterations, cam.inv_view());
                ControlFlow::Continue
            }
            WindowEvent::KeyboardInput { input, .. } => match input.state {
                ElementState::Pressed => match input.virtual_keycode {
                    Some(VirtualKeyCode::Add) => {
                        iterations *= 2;
                        draw(&display, &vertices, &indices, current_shaders, iterations, cam.inv_view());
                        ControlFlow::Continue
                    }
                    Some(VirtualKeyCode::Subtract) => {
                        iterations /= 2;
                        draw(&display, &vertices, &indices, current_shaders, iterations, cam.inv_view());
                        ControlFlow::Continue
                    }
                    Some(VirtualKeyCode::M) => {
                        current_shaders = &mandlebrot_shaders;
                        draw(&display, &vertices, &indices, current_shaders, iterations, cam.inv_view());
                        ControlFlow::Continue
                    }
                    Some(VirtualKeyCode::J) => {
                        current_shaders = &julia_shaders;
                        draw(&display, &vertices, &indices, current_shaders, iterations, cam.inv_view());
                        ControlFlow::Continue
                    }
                    Some(virtual_key_code) => {
                        cam.track(virtual_key_code);
                        draw(&display, &vertices, &indices, current_shaders, iterations, cam.inv_view());
                        ControlFlow::Continue
                    }
                    _ => ControlFlow::Continue,
                },
                ElementState::Released => ControlFlow::Continue,
            },
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
    iterations: i32,
    inv_view: [[f32; 3]; 3]
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
            &uniform! { iter: iterations, inv_view: inv_view },
            &draw_parameters,
        )
        .unwrap();
    target.finish().unwrap();
}

struct Camera {
    pos_x : f32,
    pos_y : f32,
    zoom: f32,
}

impl Camera {
    pub fn new() -> Self{
        Camera {
            pos_x : -0.5,
            pos_y : 0.0,
            zoom: 1.0,
        }
    }

    pub fn inv_view(&self) -> [[f32;3];3] {
        // Inverse view matrix, transforms from canvas space, to the space of the coordinate system.
        [
            [1. / self.zoom, 0., 0.],
            [0., 1. / self.zoom, 0.],
            [self.pos_x, self.pos_y, 1.],
        ]
    }

    pub fn track(&mut self, vkc: VirtualKeyCode) {
        match vkc {
            VirtualKeyCode::Period => self.zoom /= 1.02,
            VirtualKeyCode::Comma => self.zoom *= 1.02,
            _ => ()
        };

        let trans = match vkc {
            VirtualKeyCode::Up => (0., 1.),
            VirtualKeyCode::Down => (0., -1.),
            VirtualKeyCode::Left => (-1., 0.),
            VirtualKeyCode::Right => (1., 0.),
            _ => (0., 0.)
        };
        self.pos_x += trans.0 * 0.1 / self.zoom;
        self.pos_y += trans.1 * 0.1 / self.zoom;
    }
}