use winit::event::{ WindowEvent, Event };
use image;
#[derive(Default)]
pub struct Konway
{
    num: u32,
}

impl Konway
{
    pub fn new() -> Self
    {
        Self::default()
    }

    pub fn init()
    {
        let image = image::load(std::io::Cursor::new(&include_bytes!("/path/to/image.png")),
                                image::ImageFormat::Png).unwrap().to_rgba8();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

    }
    pub fn tick(&mut self)
    {
        self.num += 1;
        println!("{}", self.num);
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