use std::collections::HashMap;
use std::f32::consts::PI;

use ggez::graphics::Rect;
use ggez::nalgebra::Point2;
use nphysics2d::material::{BasicMaterial, MaterialHandle};
use nphysics2d::nalgebra::base::Vector2;
use nphysics2d::nalgebra::Isometry2;
use nphysics2d::ncollide2d::shape::{Ball, Cuboid, ShapeHandle};
use nphysics2d::object::{BodyStatus, ColliderDesc, RigidBodyDesc};
use specs::{Builder, Entity, World, WorldExt};

use crate::components::{Animation, AnimationParams, AnimationStates, Body, CameraTarget, FeetSensor, Jumper, Player, Sprite, Transform, BasicAI};
use crate::engine::physics::PhysicsWorld;

pub struct Mario;

impl Mario {
    pub fn add(
        world: &mut World,
        position: Point2<f32>,
        physics_world: &mut PhysicsWorld,
    ) -> Entity {
        let rigid_body = RigidBodyDesc::new()
            .position(Isometry2::new(Vector2::new(position.x, position.y), 0.0))
            .mass(1.0)
            .max_linear_velocity(1000.0)
            .kinematic_rotations(true)
            .build();

        let shape = ShapeHandle::new(Ball::new(8.0));
        let collider_desc =
            ColliderDesc::new(shape).material(MaterialHandle::new(BasicMaterial::new(0.0, 0.0)));

        let body = physics_world.insert_body(rigid_body, collider_desc);

        let rect_shape = ShapeHandle::new(Cuboid::new(Vector2::<f32>::new(7.0, 1.0)));
        let sensor_desc = ColliderDesc::new(rect_shape).translation(Vector2::new(0.0, -10.0)).user_data("feet");

        let sensor_handle = physics_world.insert_sensor(&body.rigid_body_handle, sensor_desc);

        let mut animation = Animation::default();

        animation.speed = 8.0;
        animation.speed_factor = 0.1;

        animation.animations.insert(
            AnimationStates::Idle,
            AnimationParams {
                frame: 0.0,
                frame_count: 1,
                start_frame: 0,
            },
        );

        animation.animations.insert(
            AnimationStates::Moving,
            AnimationParams {
                frame: 0.0,
                frame_count: 3,
                start_frame: 1,
            },
        );

        animation.animations.insert(
            AnimationStates::Jumping,
            AnimationParams {
                frame: 0.0,
                frame_count: 1,
                start_frame: 5,
            },
        );

        animation.animations.insert(
            AnimationStates::Drag,
            AnimationParams {
                frame: 0.0,
                frame_count: 1,
                start_frame: 4,
            },
        );

        world
            .create_entity()
            .with(Sprite {
                src: Rect::new(0.0, 0.0, 0.167, 0.33),
                width: 32f32,
                height: 16f32,
                repeat: Vector2::new(1, 1),
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
            .with(FeetSensor {
                collider_handle: sensor_handle,
                on_floor: false,
            })
            .with(Jumper::default())
            .build()
    }
}

pub struct Brick;

impl Brick {
    pub fn add(
        world: &mut World,
        position: Point2<f32>,
        block_count: Vector2<u32>,
        physics_world: &mut PhysicsWorld,
    ) -> Entity {
        let (width, height) = (16f32, 16f32);

        let rigid_body = RigidBodyDesc::new()
            .position(Isometry2::new(Vector2::new(position.x, position.y), 0.0))
            .status(BodyStatus::Static)
            .mass(2000.0)
            .build();

        let shape = ShapeHandle::new(Cuboid::new(Vector2::new(block_count.x as f32 * width / 2.0, block_count.y as f32 * height / 2.0)));
        let collider_desc = ColliderDesc::new(shape)
            .density(100.0)
            .translation(Vector2::new((block_count.x as f32 * width) / 2.0 - width / 2.0, (block_count.y as f32 * -height) / 2.0 + height / 2.0))
            .material(MaterialHandle::new(BasicMaterial::new(0.0, 0.2))).user_data("brick");

        let body = physics_world.insert_body(rigid_body, collider_desc);

        world
            .create_entity()
            .with(Sprite {
                src: Rect::new(0.0, 0.34, 0.1, 0.33),
                width,
                height,
                repeat: block_count,
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

pub struct KoopaTroopa;

impl KoopaTroopa {
    pub fn add(
        world: &mut World,
        position: Point2<f32>,
        physics_world: &mut PhysicsWorld,
    ) -> Entity {
        let rigid_body = RigidBodyDesc::new()
            .position(Isometry2::new(Vector2::new(position.x, position.y), 0.0))
            .mass(100.0)
            .max_linear_velocity(1000.0)
            .kinematic_rotations(true)
            .build();

        let shape = ShapeHandle::new(Ball::new(6.0));
        let collider_desc =
            ColliderDesc::new(shape).material(MaterialHandle::new(BasicMaterial::new(0.0, 0.0))).translation(Vector2::new(0.0, -10.0));

        let body = physics_world.insert_body(rigid_body, collider_desc);

        let rect_shape = ShapeHandle::new(Cuboid::new(Vector2::<f32>::new(3.0, 1.0)));
        let sensor_desc = ColliderDesc::new(rect_shape).translation(Vector2::new(0.0, -16.0)).user_data("feet");

        let sensor_handle = physics_world.insert_sensor(&body.rigid_body_handle, sensor_desc);

        let mut animation = Animation::default();

        animation.speed = 10.0;
        animation.speed_factor = 0.1;

        animation.animations.insert(
            AnimationStates::Idle,
            AnimationParams {
                frame: 3.0,
                frame_count: 1,
                start_frame: 3,
            },
        );

        animation.animations.insert(
            AnimationStates::Moving,
            AnimationParams {
                frame: 3.0,
                frame_count: 2,
                start_frame: 3,
            },
        );


        world
            .create_entity()
            .with(Sprite {
                src: Rect::new(0.67, 0.34, 0.167, 0.67),
                width: 32f32,
                height: 32f32,
                repeat: Vector2::new(1, 1),
            })
            .with(Transform {
                position,
                rotation: 0.0,
                scale: Vector2::new(1.0, 1.0),
            })
            .with(body)
            .with(animation)
            .with(FeetSensor {
                collider_handle: sensor_handle,
                on_floor: false,
            })
            .with(BasicAI)
            .build()
    }
}
