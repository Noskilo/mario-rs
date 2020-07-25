use crate::components::{DynamicBody, CameraTarget, Player, Sprite, StaticBody, Transform};
use ggez::{
    graphics::Rect,
    nalgebra::{Point2, Vector2},
};
use specs::{Builder, Entity, World, WorldExt};

pub struct Mario;

impl Mario {
    pub fn add(world: &mut World, position: Point2<f32>) -> Entity {
        world
            .create_entity()
            .with(Sprite {
                src: Rect::new(0.0, 0.0, 0.2, 0.33),
                width: 32f32,
                height: 16f32,
            })
            .with(Transform {
                position,
                rotation: 0.0,
                scale: Vector2::new(1.0, 1.0),
            })
            .with(DynamicBody {
                velocity: Vector2::new(0.0, 0.0),
                width: 14f32,
                height: 16f32,
            })
            .with(Player::default())
            .with(CameraTarget::default())
            .build()
    }
}

pub struct Brick;

impl Brick {
    pub fn add(world: &mut World, position: Point2<f32>) -> Entity {
        world
            .create_entity()
            .with(Sprite {
                src: Rect::new(0.0, 0.34, 0.1, 0.33),
                width: 16f32,
                height: 16f32,
            })
            .with(Transform {
                position,
                rotation: 0.0,
                scale: Vector2::new(1.0, 1.0),
            })
            .with(StaticBody {
                width: 16f32,
                height: 16f32
            })
            .build()
    }
}
