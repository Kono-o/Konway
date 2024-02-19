#![macro_use]
#![allow(unused_assignments)]
extern crate glium;
use glium::uniform;
use std::sync::Arc;
use game_loop::game_loop;
use winit::event::{ElementState, Event, WindowEvent};
use winit::keyboard::Key;
use winit::platform::modifier_supplement::KeyEventExtModifierSupplement;
use glium::
{ Surface,
  uniforms::{ MagnifySamplerFilter, MinifySamplerFilter }
};

mod konway;
mod polygon;
mod shaders;

const WINDOW_TITLE: &str = "Konway";
const WINDOW_SIZE: u32 = 800;

const TICK_RATE: u32 = 20;

const TRANS_MATRIX: [[f32; 4]; 4] =
[
    [1.0,0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0 , 0.0, 0.0, 1.0f32],
];

pub fn main()
{
    let mut tick: u32 = 0;

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
    let behavior = glium::uniforms::SamplerBehavior
    {
        minify_filter: MinifySamplerFilter::Nearest,
        magnify_filter: MagnifySamplerFilter::Nearest,
        ..Default::default()
    };

    //new game impl
    let mut gol = konway::Konway::new();
    let mut texture = glium::texture::Texture2d::new(&display, gol.init(TICK_RATE as u8)).unwrap();
    //game-loop with winit event loop
     game_loop(event_loop, Arc::new(window), gol, TICK_RATE, 0.1,
       move |g|
        {
            /*logic functions (update frequency dependent on tick-rate)*/

            //game update
            if tick > TICK_RATE || tick == 0 {texture = glium::texture::Texture2d::new(&display, g.game.tick()).unwrap();}
            tick += 1;

            //ogl static data
            let uniforms = uniform! {matrix: TRANS_MATRIX,tex: glium::uniforms::Sampler(&texture, behavior),};

            // frame buffer
            let mut frame = display.draw();
            // fill frame with black
            frame.clear_color(0.0, 0.0, 0.0, 1.0);
            //draw square
            frame.draw(&points, &indices,
                       &shaders, &uniforms,
                       &Default::default()).expect("square draw");
            //finish draw
             frame.finish().expect("frame finish");
        },
        |_g|
        {
            /*render functions (independent of tick-rate,
             will try to update as fast as possible)
             currently unimplemented*/
        },
        move |g, event|
        {
            /*window event handling*/
            match event
            {
                Event::WindowEvent { event, .. } =>
                    match event
                    {
                        WindowEvent::CloseRequested => g.exit(),
                        WindowEvent::KeyboardInput { event, .. } =>
                            {
                                if event.state == ElementState::Pressed && !event.repeat
                                {
                                    match event.key_without_modifiers().as_ref()
                                    {
                                        Key::Character("1") => g.game.pause(),
                                        _ => {},
                                    }
                                }
                            }
                        _ => {},
                    },
                _ => {},
            }
        }
     ).expect("game loop");
}
