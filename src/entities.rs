use crate::components::{Transform, Sprite};
use ggez::{graphics::Rect, nalgebra::{Vector2, Point2}};
use specs::{Entity, World, WorldExt, Builder};

pub struct Mario;

impl Mario {
    pub fn add(world: &mut World, position: Point2<f32>) -> Entity {
        world
            .create_entity()
            .with(Sprite {
                src: Rect::new(0.0, 0.0, 0.2, 1.0),
                width: 18f32,
                height: 16f32,
            })
            .with(Transform {
                position,
                rotation: 0.0,
                scale: Vector2::new(1.0, 1.0),
            })
            .build()
    }
}
