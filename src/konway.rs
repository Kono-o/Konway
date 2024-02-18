use glium::texture::RawImage2d;
use winit::event::{WindowEvent, Event };
use image;
use image::{Rgba, RgbaImage};
use rand::{Rng, thread_rng};

const CANVAS_SIZE: u32 = 50;

const PROBABILTY: u32 =  8;

const BRIGHTNESS: u8 = 255;
const WHITE: Rgba<u8> = Rgba([BRIGHTNESS, BRIGHTNESS, BRIGHTNESS, BRIGHTNESS]);
const BLACK: Rgba<u8> = Rgba([0, 0, 0, BRIGHTNESS]);

#[derive(Default)]
pub struct Konway
{
    canvas: RgbaImage,
}

impl Konway
{
    pub fn new() -> Self
    {
        Self::default()
    }

    pub fn generate(&mut self) -> RawImage2d<'static, u8>
    {
        self.canvas = RgbaImage::new(CANVAS_SIZE, CANVAS_SIZE);

        for (_x, _y, pixel) in self.canvas.enumerate_pixels_mut()
        {
            let rng = thread_rng().gen_ratio(1, PROBABILTY);
            if rng { *pixel = WHITE; }
            else { *pixel = BLACK; }

        }
        let image = RawImage2d::from_raw_rgba_reversed
            (&<image::ImageBuffer<Rgba<u8>, Vec<u8>> as Clone>::clone(&self.canvas).into_raw(), (CANVAS_SIZE,CANVAS_SIZE));

        return image;
    }

    pub fn tick(&mut self)  -> RawImage2d<'static, u8>
    {
        let buffr = &<image::ImageBuffer<Rgba<u8>, Vec<u8>> as Clone>::clone(&self.canvas);
        let mut neigbours: u8;

        for (x, y, pixel) in self.canvas.enumerate_pixels_mut()
        {
            neigbours = 0;
            //border prevention
            if !(x == 0 || x == CANVAS_SIZE-1 || y == 0 || y == CANVAS_SIZE-1)
            {
                //println!("{} {}", x, y);
                //cardinal neighbours
                if buffr[(x+1,y)] == WHITE { neigbours += 1; }
                if buffr[(x-1,y)] == WHITE { neigbours += 1; }
                if buffr[(x,y+1)] == WHITE { neigbours += 1; }
                if buffr[(x,y+1)] == WHITE { neigbours += 1; }
                //diagonal neighbours
                if buffr[(x+1,y+1)] == WHITE { neigbours += 1; }
                if buffr[(x-1,y-1)] == WHITE { neigbours += 1; }
                if buffr[(x-1,y+1)] == WHITE { neigbours += 1; }
                if buffr[(x+1,y-1)] == WHITE { neigbours += 1; }

                if *pixel == WHITE
                {
                    if neigbours < 2 || neigbours > 3 { *pixel = BLACK; }
                }
                else { if neigbours == 3 { *pixel = WHITE; } }
            }
            else { *pixel = BLACK }
        };

        let image = RawImage2d::from_raw_rgba_reversed
            (&<image::ImageBuffer<Rgba<u8>, Vec<u8>> as Clone>::clone(&self.canvas).into_raw(), (CANVAS_SIZE,CANVAS_SIZE));
        return image
    }

    pub fn win_close(&self, event: &Event<()>) -> bool
    {
        match event
        {
            Event::WindowEvent { event, .. } =>
            match event
            {
                WindowEvent::CloseRequested =>
                {
                    return false;
                },
                _ => {},
            },
            _ => {},
        }
        true
    }
}