#[macro_use]
extern crate glium;

use glium::{Frame, Surface};

const WINDOW_TITLE: &str = "Konway";
const WINDOW_SIZE: u32 = 500;

fn main()
{
    // winit event loop
    let event_loop = winit::event_loop::EventLoopBuilder::new()
        .build()
        .expect("event loop creation");

    // glutin window and ogl context
    let context_window = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title(WINDOW_TITLE)
        .with_inner_size(WINDOW_SIZE,WINDOW_SIZE);

    // glium display
    let (_window, display) = context_window.build(&event_loop);

    //TRIANGLE :)
    #[derive(Copy, Clone)]
    struct Point
    {
        position: [f32; 2],
    }
    implement_vertex!(Point, position);

    let vert1 = Point { position: [-0.5, -0.5] };
    let vert2 = Point { position: [ 0.0, 0.5] };
    let vert3 = Point { position: [ 0.5, -0.25] };
    let triangle = vec![vert1, vert2, vert3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &triangle).expect("vertex buffer");
    //set indices to no for drawing only disjointed triangles
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader =
    r#"
        #version 140
        in vec2 position;

        uniform float x;
        uniform float y;

        void main()
        {
            vec2 pos = position;
            pos.x += x;
            pos.y += y;
            gl_Position = vec4(pos, 0.0, 1.0);
        }
    "#;

    let fragment_shader =
    r#"
        #version 140
        out vec4 color;

        void main() {
        color = vec4(1.0, 1.0, 1.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader, fragment_shader, None)
                  .expect("program");

    //event handling
    let mut x_axis: f32 = 0.0;
    let mut y_axis: f32 = 0.0;
    event_loop.run(move |event, window_target|
        {
            //println!("{:?}", event);

            match event
            {
                winit::event::Event::WindowEvent { event, .. } =>
                match event
                {
                    winit::event::WindowEvent::CloseRequested => window_target.exit(),
                    winit::event::WindowEvent::Resized(window_size) => display.resize(window_size.into()),

                    winit::event::WindowEvent::RedrawRequested =>
                    {
                        x_axis += 0.02;
                        y_axis += 0.01;
                        let x_sine = x_axis.sin() * 0.5;
                        let y_sine = y_axis.sin() * 0.5;

                        // frame buffer
                        let mut frame :Frame = display.draw();

                        // fill frame with black
                        frame.clear_color(0.0, 0.0, 0.0, 1.0);

                        //draw triangle
                        frame.draw(&vertex_buffer, &indices,
                                   &program, &uniform! {x: x_sine,y: y_sine},
                                   &Default::default())
                                   .expect("triangle draw");

                        //finish draw
                        frame.finish().expect("frame finish");
                    }
                    _ => (),
                },
                winit::event::Event::AboutToWait => _window.request_redraw(),
                _ => (),
            };
        }).expect("event loop run");
}