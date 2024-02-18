#[macro_use]
extern crate glium;
use glium::
{ Surface,
  uniforms::{ MagnifySamplerFilter, MinifySamplerFilter }
};

use std::sync::Arc;
use game_loop::game_loop;

mod konway;
mod polygon;
mod shaders;

const WINDOW_TITLE: &str = "Konway";
const WINDOW_SIZE: u32 = 800;

const TICK_RATE: u32 = 5;

const TRANS_MATRIX: [[f32; 4]; 4] =
[
    [1.0,0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0 , 0.0, 0.0, 1.0f32],
];

pub fn main()
{
    //winit event loop
    let event_loop = winit::event_loop::EventLoopBuilder::new().build()
        .expect("event loop creation");

    //glutin window + ogl context + glium display
    let window_builder = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title(WINDOW_TITLE)
        .with_inner_size(WINDOW_SIZE,WINDOW_SIZE);
    let (window, display) = window_builder.build(&event_loop);
    window.set_resizable(false);

    //polygon data
    let points = glium::VertexBuffer::new(&display, &polygon::POINTS)
        .expect("polygon positions");
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    //shader code
    let vert_shader: &str = shaders::VERT;
    let frag_shader: &str = shaders::FRAG;
    let shaders = glium::Program::from_source
        (&display, vert_shader, frag_shader, None)
        .expect("shaders");

    //ogl texture filtering
    let _behavior = glium::uniforms::SamplerBehavior
    {
        minify_filter: MinifySamplerFilter::Nearest,
        magnify_filter: MagnifySamplerFilter::Nearest,
        ..Default::default()
    };

    //new game impl
    let gol = konway::Konway::new();

    //game-loop with winit event loop
     game_loop(event_loop, Arc::new(window), gol, TICK_RATE, 0.1,
       move |g|
        {
            /*logic functions (dependent on tick-rate)*/
            let uniforms = uniform!
            {
                matrix: TRANS_MATRIX,
                //glium::uniforms::Sampler(&texture, behavior)
            };
            //konway tick
            let _ = g.game.tick();
            // frame buffer
            let mut frame = display.draw();
            // fill frame with black
            frame.clear_color(0.0, 0.0, 0.0, 1.0);
            //draw square
            frame.draw(&points, &indices,
                       &shaders, &uniforms,
                       &Default::default())
                .expect("square draw");
            //finish draw
            frame.finish().expect("frame finish");

        },
        |_g|
        {
            /*render functions (independent of tick-rate,
             will try to render as fast as possible)*/
        },
        |g, event|
        {
            //window event handling
            if !g.game.win_close(event) { g.exit();}
        })
        .expect("game loop");
}
