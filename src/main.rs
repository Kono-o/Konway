#[macro_use]
extern crate glium;
use glium::Surface;
use winit;

fn main()
{
    // 1. winit::EventLoop for handling events.
    let event_loop = winit::event_loop::EventLoopBuilder::new()
        .build().expect("err: event loop");

    // 2. glutin context and glium Display
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .build(&event_loop);

    #[derive(Copy, Clone)]
    struct Point
    {
        position: [f32; 2],
    }
    implement_vertex!(Point, position);

    let vert1 = Point { position: [0.0, 0.0] };
    let vert2 = Point { position: [ 0.0,  0.5] };
    let vert3 = Point { position: [ 0.5, -0.0] };
    let triangle = vec![vert1, vert2, vert3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &triangle).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src =
    r#"
        #version 140
        in vec2 position;

        void main() {
        gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src =
    r#"
        #version 140
        out vec4 color;

        void main() {
        color = vec4(1.0, 1.0, 1.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut frame = display.draw();
    frame.clear_color(0.1, 0.0, 0.75, 1.0);
    //draw triangle
    frame.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
                &Default::default()).unwrap();
    frame.finish().unwrap();

    let _ = event_loop.run(move |event, window_target|
        {
        match event
        {
            winit::event::Event::WindowEvent { event, .. } => match event
            {
                winit::event::WindowEvent::CloseRequested => window_target.exit(),
                _ => (),
            },
            _ => (),
        };
    });
}