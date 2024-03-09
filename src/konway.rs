use glium::texture::RawImage2d;
use image;
use image::{Rgba, RgbaImage};
use rand::{Rng, thread_rng};

const CANVAS_SIZE: u32 = 250;

const PROBABILTY: u32 =  32;

const MAX: u8 = 255;
const FADE: f32 = 0.96;

const WHITE: Rgba<u8> = Rgba([MAX, MAX, MAX, MAX]);
const BLACK: Rgba<u8> = Rgba([0, 0, 0, MAX]);

#[derive(Default)]
pub struct Konway
{
    tps: u8,
    game_tick: u32,
    paused: bool,
    canvas: RgbaImage,
}

impl Konway
{
    pub fn new() -> Self {Self::default()}

    pub fn pause(&mut self)
    {
        self.paused = !self.paused;
        /*if self.paused {println!("paused");}
        else {println!("unpaused");}*/
    }

    pub fn forward(&mut self)
    {
        if self.paused
        {
            self.paused = false;
            self.tick();
            self.paused = true;
        }
    }
    pub fn init(&mut self, tps: u8, pause: bool) -> RawImage2d<'static, u8>
    {
        self.tps = tps;
        self.paused = pause;
        self.canvas = RgbaImage::new(CANVAS_SIZE, CANVAS_SIZE);
        
        let probability = thread_rng().gen_range(2..PROBABILTY);
        for (_x, _y, pixel) in self.canvas.enumerate_pixels_mut()
        {
            let rng = thread_rng().gen_ratio(1, probability);
            if rng { *pixel = WHITE; }
            else { *pixel = BLACK; }
        }
        let image = RawImage2d::from_raw_rgba_reversed
            (&<image::ImageBuffer<Rgba<u8>, Vec<u8>> as Clone>::clone(&self.canvas).into_raw(), (CANVAS_SIZE,CANVAS_SIZE));

        return image;
    }

    pub fn tick(&mut self)  -> RawImage2d<'static, u8>
    {
        if !self.paused
        {
            self.game_tick += 1;
            //println!("game tick: {}", self.game_tick);

            let buffr = &<image::ImageBuffer<Rgba<u8>, Vec<u8>> as Clone>::clone(&self.canvas);
            let _neigbours: u8 = 0;

            for (x, y, pixel) in self.canvas.enumerate_pixels_mut()
            {
                //border prevention
                if !(x == 0 || x == CANVAS_SIZE-1 || y == 0 || y == CANVAS_SIZE-1)
                {
                    let mut neighbours = 0;
                    //cardinals
                    if buffr[(x-1,y)] == WHITE {neighbours += 1;}
                    if buffr[(x+1,y)] == WHITE {neighbours += 1;}
                    if buffr[(x,y-1)] == WHITE {neighbours += 1;}
                    if buffr[(x,y+1)] == WHITE {neighbours += 1;}
                    //diagonals
                    if buffr[(x-1,y-1)] == WHITE {neighbours += 1;}
                    if buffr[(x+1,y+1)] == WHITE {neighbours += 1;}
                    if buffr[(x+1,y-1)] == WHITE {neighbours += 1;}
                    if buffr[(x-1,y+1)] == WHITE {neighbours += 1;}

                    if buffr[(x,y)] == WHITE
                    {
                        if neighbours < 2 || neighbours > 3 {*pixel = BLACK}
                    }
                    else { if neighbours == 3 {*pixel = WHITE}}
                }
                else {*pixel = BLACK;}
            };
            for (x, y, pixel) in self.canvas.enumerate_pixels_mut()
            {
                if *pixel != WHITE
                {
                    if *pixel == BLACK
                    {
                        *pixel = Rgba([
                            (f32::from(buffr[(x,y)].0[0])* FADE * FADE *0.99)as u8,
                            (f32::from(buffr[(x,y)].0[1])* FADE * FADE)as u8,
                            (f32::from(buffr[(x,y)].0[2])* FADE)as u8,
                            MAX]);
                    }
                    else
                    {
                        *pixel = Rgba([
                            (f32::from(pixel.0[0])* FADE * FADE *0.99)as u8,
                            (f32::from(pixel.0[1])* FADE * FADE)as u8,
                            (f32::from(pixel.0[2])* FADE)as u8,
                            MAX]);
                    }
                }
            }
        }
        let image = RawImage2d::from_raw_rgba_reversed
            (&<image::ImageBuffer<Rgba<u8>, Vec<u8>> as Clone>::clone(&self.canvas).into_raw(), (CANVAS_SIZE,CANVAS_SIZE));
        return image
    }
}