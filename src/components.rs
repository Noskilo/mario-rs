use nalgebra::Vector2;
use sdl2::rect::Rect;

#[derive(Clone, Debug)]
pub struct Visual {
    pub texture_id: String,
    pub src_rect: Rect
}

#[derive(Clone, Copy, Debug)]
pub struct Transform {
    pub position: Vector2<f32>,
    pub rotation: f32,
    pub scale: f32,
}
