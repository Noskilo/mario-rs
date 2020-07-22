use ggez::{
    graphics, nalgebra::{Point2, Vector2},
};
use specs::Component;
use specs::VecStorage;

#[derive(Clone, Copy, Debug)]
pub struct Transform {
    pub position: Point2<f32>,
    pub rotation: f32,
    pub scale: Vector2<f32>,
}

impl Component for Transform {
    type Storage = VecStorage<Self>;
}

#[derive(Clone, Copy, Debug)]
pub struct Sprite {
    pub src: graphics::Rect,
    pub width: f32,
    pub height: f32,
}

impl Component for Sprite {
    type Storage = VecStorage<Self>;
}
