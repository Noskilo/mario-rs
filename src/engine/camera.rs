use ggez::{
    graphics::{self, spritebatch::SpriteBatch},
    nalgebra::{Point2, Vector2},
    Context, GameResult,
};

use crate::util::lerp::Lerp;
use graphics::{DrawParam, Mesh};
use std::collections::VecDeque;

pub struct Camera {
    pub position: Point2<f32>,
    pub zoom: f32,
    target_position: Point2<f32>,
    target_zoom: f32,
}

impl Camera {
    pub fn render(&self, ctx: &mut Context, batch: &SpriteBatch) -> GameResult<()> {
        let param = graphics::DrawParam::new()
            .offset(Point2::new(0.5, 0.5))
            .scale(Vector2::new(self.zoom, self.zoom));

        graphics::draw(ctx, batch, param)?;

        Ok(())
    }

    pub fn debug_render(
        &self,
        ctx: &mut Context,
        debug_renderables: &mut VecDeque<Mesh>,
    ) -> GameResult<()> {
        while !debug_renderables.is_empty() {
            let mesh = debug_renderables.pop_front().unwrap();
            graphics::draw(
                ctx,
                &mesh,
                DrawParam::default()
                    .offset(Point2::new(0.5, 0.5))
                    .scale(Vector2::new(self.zoom, self.zoom)),
            )?;
        }

        Ok(())
    }

    pub fn set_target(&mut self, position: &Point2<f32>) {
        self.target_position = position.clone();
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom;
    }

    pub fn update(&mut self) {
        let ratio = 0.3;

        self.position.x = self.position.x.lerp(self.target_position.x, ratio);
        self.position.y = self.position.y.lerp(self.target_position.y, ratio);

        self.zoom = self.zoom.lerp(self.target_zoom, ratio);
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            position: Point2::new(0.0, 0.0),
            zoom: 4.0,
            target_zoom: 4.0,
            target_position: Point2::new(0.0, 0.0),
        }
    }
}
