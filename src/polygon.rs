#[derive(Copy, Clone)]
pub struct Point
{
    position: [f32; 2],
    tex_coords: [f32; 2],
}
implement_vertex!(Point, position, tex_coords);

pub const POINTS: [Point; 6] =
[
    Point { position: [-1.0, -1.0], tex_coords: [0.0, 0.0] },
    Point { position: [ 1.0, -1.0], tex_coords: [1.0, 0.0] },
    Point { position: [ 1.0,  1.0], tex_coords: [1.0, 1.0] },

    Point { position: [ 1.0,  1.0], tex_coords: [1.0, 1.0] },
    Point { position: [-1.0,  1.0], tex_coords: [0.0, 1.0] },
    Point { position: [-1.0, -1.0], tex_coords: [0.0, 0.0] },
];