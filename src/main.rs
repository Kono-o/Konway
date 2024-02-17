#[macro_use]
extern crate glium;
use glium::{Frame, Surface};

mod model;

const WINDOW_TITLE: &str = "Konway";
const WINDOW_SIZE: u32 = 500;

const LIGHT_DIR: [f32; 3] = [-1.0, 0.4, 0.9f32];

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

    //model data
    let positions = glium::VertexBuffer::new(&display, &model::VERTICES).expect("model position");
    let normals = glium::VertexBuffer::new(&display, &model::NORMALS).expect("model normals");
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &model::INDICES).expect("model indices");

    let vertex_shader =
    r#"
        #version 150

        in vec3 position;
        in vec3 normal;

        out vec3 v_normal;

        uniform mat4 matrix;

        void main()
        {
            v_normal = transpose(inverse(mat3(matrix))) * normal;
            gl_Position = matrix * vec4(position, 1.0);
        }
    "#;

    let fragment_shader =
    r#"
        #version 140

        in vec3 v_normal;
        out vec4 color;
        uniform vec3 u_light;

        void main()
        {
            float brightness = dot(normalize(v_normal), normalize(u_light));

            vec3 def_color = vec3(0.58, 0.83, 1);
            vec3 shadow_col = vec3(0.08, 0.5, 0.78);

             color = vec4(mix(shadow_col, def_color, brightness), 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader, fragment_shader, None)
                  .expect("program");

    //event handling
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
                        let uniforms = uniform!
                        {
                            matrix:
                            [
                                [0.01, 0.0, 0.0, 0.0],
                                [0.0, 0.01 , 0.0, 0.0],
                                [0.0, 0.0, 0.01, 0.0],
                                [0.0 , 0.0, 0.0, 1.0f32],
                            ],
                            u_light: LIGHT_DIR
                        };

                        // frame buffer
                        let mut frame :Frame = display.draw();

                        // fill frame with black
                        frame.clear_color(0.0, 0.0, 0.0, 1.0);

                        //draw triangle
                        frame.draw((&positions, &normals), &indices, &program, &uniforms,
                                    &Default::default()).expect("triangle draw");

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