use crate::components::{CameraTarget, Player, Sprite, Transform, Body, Animation, AnimationStates, AnimationParams};
use ggez::{
    graphics::Rect
};
use specs::{Builder, Entity, World, WorldExt};
use crate::engine::physics::PhysicsWorld;
use nphysics2d::object::{RigidBodyDesc, BodyStatus, ColliderDesc};
use nphysics2d::nalgebra::base::{Vector2};
use ggez::nalgebra::Point2;
use nphysics2d::nalgebra::Isometry2;
use nphysics2d::ncollide2d::shape::{Cuboid, Ball, ShapeHandle};
use nphysics2d::material::{MaterialHandle, BasicMaterial};
use std::collections::HashMap;
use std::f32::consts::PI;


pub struct Mario;

impl Mario {
    pub fn add(world: &mut World, position: Point2<f32>, physics_world: &mut PhysicsWorld) -> Entity {
        let rigid_body = RigidBodyDesc::new()
            .position(Isometry2::new(Vector2::new(position.x, position.y), 0.0))
            .mass(1.0)
            .build();

        let shape = ShapeHandle::new(Ball::new(8.0));
        let collider_desc = ColliderDesc::new(shape)
            .material(MaterialHandle::new(BasicMaterial::new(0.0, 0.2)));

        let body = physics_world.insert_body(rigid_body, collider_desc);

        let mut animation = Animation::default();

        animation.animations.insert(AnimationStates::Idle, AnimationParams {
            frame: 0.0,
            speed: 0.0,
            frame_count: 1,
            start_frame: 0,
        });


        animation.animations.insert(AnimationStates::Moving, AnimationParams {
            frame: 0.0,
            speed: 8.0,
            frame_count: 3,
            start_frame: 1,
        });

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
            .with(Player::default())
            .with(CameraTarget::default())
            .with(body)
            .with(animation)
            .build()
    }
}

pub struct Brick;

impl Brick {
    pub fn add(world: &mut World, position: Point2<f32>, physics_world: &mut PhysicsWorld) -> Entity {
        let rigid_body = RigidBodyDesc::new()
            .position(Isometry2::new(Vector2::new(position.x, position.y), 0.0))
            .status(BodyStatus::Static)
            .build();

        let shape = ShapeHandle::new(Cuboid::new(Vector2::new(8.0, 8.0)));
        let collider_desc = ColliderDesc::new(shape)
            .density(100.0)
            .material(MaterialHandle::new(BasicMaterial::new(0.0, 0.8)));

        let body = physics_world.insert_body(rigid_body, collider_desc);

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
            .with(body)
            .build()
    }
}
