use ggez::{
    graphics,
    mint::{Point2, Vector2},
};
use graphics::spritebatch::SpriteBatch;
use specs::Component;
use specs::VecStorage;

#[derive(Clone, Copy, Debug)]
pub struct Transform {
    pub position: Point2<f32>,
    pub rotation: f32,
    pub scale: Vector2<f32>,
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

impl Sprite {
    pub fn draw(&self, batch: &mut SpriteBatch, transform: &Transform) {
        batch.add(
            graphics::DrawParam::new()
                .src(self.src)
                .scale(transform.scale)
                .dest(transform.position),
        );
    }
}
