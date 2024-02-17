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
        pos: [f32; 2],
        col: [f32; 3],
    }
    implement_vertex!(Point, pos, col);

    let vert1 = Point { pos: [-0.5, -0.5], col: [1.0, 0.0, 0.0] };
    let vert2 = Point { pos: [ 0.0, 0.5], col: [0.0, 1.0, 0.0]  };
    let vert3 = Point { pos: [ 0.5, -0.25], col: [0.0, 0.0, 1.0]  };
    let triangle = vec![vert1, vert2, vert3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &triangle).expect("vertex buffer");
    //set indices to no for drawing only disjointed triangles
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader =
    r#"
        #version 140

        in vec2 pos;
        in vec3 col;

        out vec3 vertex_color;

        uniform mat4 matrix;

        void main()
        {
            vertex_color = col;
            gl_Position = matrix * vec4(pos, 0.0, 1.0);
        }
    "#;

    let fragment_shader =
    r#"
        #version 140

        in vec3 vertex_color;
        out vec4 color;

        void main() {
        color = vec4(vertex_color, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader, fragment_shader, None)
                  .expect("program");

    //event handling
    let mut time: f32 = 0.0;
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
                        time += 0.005;
                        let uniforms = uniform!
                        {
                            matrix:
                            [
                                [time.cos(), time.sin(), 0.0, 0.0],
                                [-time.sin(), time.cos(), 0.0, 0.0],
                                [0.0, 0.0, 1.0, 0.0],
                                [0.0 , 0.0, 0.0, 1.0f32],
                            ]
                        };

                        // frame buffer
                        let mut frame :Frame = display.draw();

                        // fill frame with black
                        frame.clear_color(0.0, 0.0, 0.0, 1.0);

                        //draw triangle
                        frame.draw(&vertex_buffer, &indices,
                                   &program, &uniforms,
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