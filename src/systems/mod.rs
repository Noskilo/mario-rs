use std::f32::consts::PI;

use ggez::{
    Context,
    event::KeyCode,
    graphics,
    graphics::MeshBuilder,
    nalgebra::{Point2, Vector2},
};
use ggez::input::keyboard::KeyMods;
use nphysics2d::algebra::{Force2, ForceType, Velocity2};
use nphysics2d::material::BasicMaterial;
use nphysics2d::nalgebra::Isometry2;
use nphysics2d::ncollide2d::query::Proximity;
use specs::{Read, ReadStorage, System, world::Index, Write, WriteStorage};
use specs::hibitset::BitSetLike;
use specs::prelude::*;

use crate::components::{Animation, AnimationStates, Jumper, Sprite, BasicAI};
use crate::components::{Body, CameraTarget, FeetSensor, Player, Transform};
use crate::engine::{
    camera::Camera,
    game::TARGET_FPS,
    physics::PhysicsWorld,
    resources::{DebugRenderables, DeltaTime, InputEvents, Renderables},
};

pub struct RenderingSystem<'a> {
    ctx: &'a mut Context,
}

impl<'a> RenderingSystem<'a> {
    pub fn new(ctx: &'a mut Context) -> Self {
        Self { ctx }
    }
}

impl<'a> System<'a> for RenderingSystem<'a> {
    type SystemData = (
        ReadStorage<'a, Sprite>,
        ReadStorage<'a, Transform>,
        Write<'a, Renderables>,
        Write<'a, DebugRenderables>,
        Read<'a, Camera>,
    );

    fn run(
        &mut self,
        (
            sprite_storage,
            transform_storage,
            mut renderables,
            mut debug_renderables,
            camera,
        ): Self::SystemData,
    ) {
        let (width, height) = graphics::size(self.ctx);

        for (sprite, transform) in (&sprite_storage, &transform_storage).join() {
            for count_x in 0..sprite.repeat.x {
                for count_y in 0..sprite.repeat.y {
                    let pos_x = transform.position.x + count_x as f32 * sprite.width;
                    let pos_y = -transform.position.y + count_y as f32 * sprite.height;
                    let draw_param = graphics::DrawParam::new()
                        .src(sprite.src)
                        .scale(Vector2::new(transform.scale.x, transform.scale.y))
                        .dest(Point2::new(
                            (pos_x - camera.position.x) + width / (2.0 * camera.zoom),
                            (pos_y + camera.position.y) + height / (2.0 * camera.zoom),
                        ))
                        .offset(Point2::new(0.5, 0.5))
                        .rotation(transform.rotation);

                    renderables.0.push_back(draw_param);
                }
            }
        }
    }
}

pub struct PlayerControlSystem;

impl<'a> System<'a> for PlayerControlSystem {
    type SystemData = (
        WriteStorage<'a, Transform>,
        Read<'a, InputEvents>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Body>,
        Write<'a, PhysicsWorld>,
        ReadStorage<'a, FeetSensor>,
        WriteStorage<'a, Jumper>,
        WriteStorage<'a, Animation>,
    );

    fn run(
        &mut self,
        (
            mut transform_storage,
            input_events,
            player,
            body_storage,
            mut physics_world,
            feet_sensor_storage,
            mut jumper_storage,
            mut animation_storage,
        ): Self::SystemData,
    ) {
        let center_point = nphysics2d::nalgebra::Point2::new(0.0, 0.0);
        let accel = (400.0 * 1.0 / TARGET_FPS as f32) as f32;
        let mut force = Force2::linear(nphysics2d::nalgebra::Vector2::new(0.0, 0.0));
        let mut top_speed = 100.0;
        let mut friction = 0.5;

        if input_events.is_mod_active(KeyMods::SHIFT) {
            top_speed = 240.0;
        }

        for (transform, body, _, feet_sensor, jumper, animation) in (
            &mut transform_storage,
            &body_storage,
            &player,
            (&feet_sensor_storage).maybe(),
            (&mut jumper_storage).maybe(),
            (&mut animation_storage).maybe(),
        )
            .join()
        {
            let rigid_body = physics_world.bodies.get(body.rigid_body_handle);

            if let Some(rigid_body) = rigid_body {
                let current_velocity = rigid_body.velocity_at_point(0, &center_point);

                if let Some(feet_sensor) = feet_sensor {
                    if input_events.is_key_pressed(&KeyCode::D)
                        || input_events.is_key_pressed(&KeyCode::Right)
                    {
                        if current_velocity.linear.x < top_speed {
                            force.linear.x = accel;
                        }

                        transform.scale.x = transform.scale.x.abs();
                        friction = 0.0;
                    } else if input_events.is_key_pressed(&KeyCode::A)
                        || input_events.is_key_pressed(&KeyCode::Left)
                    {
                        if current_velocity.linear.x > -top_speed {
                            force.linear.x = -accel;
                        }

                        transform.scale.x = -transform.scale.x.abs();
                        friction = 0.0;
                    }

                    if let Some(jumper) = jumper {
                        if feet_sensor.on_floor {
                            jumper.jump_active = true;
                        }

                        if input_events.is_key_pressed(&KeyCode::Space) {
                            if jumper.jump_active {
                                let hold_time = input_events
                                    .key_hold_time(&KeyCode::Space)
                                    .unwrap()
                                    .elapsed()
                                    .as_secs_f32();

                                let mut jump_time = 0.2;

                                if input_events.is_mod_active(KeyMods::SHIFT) {
                                    jump_time = 0.24;
                                }

                                if hold_time < jump_time {
                                    if let Some(animation) = animation {
                                        animation.current_state = AnimationStates::Jumping;
                                    }

                                    force.linear.y = 40.0 * ((jump_time - hold_time) / jump_time);
                                    friction = 0.0;
                                }
                            }
                        } else {
                            jumper.jump_active = false;
                        }
                    }
                }
            }

            if force.linear.magnitude() > 0.0 {
                let rigid_body = physics_world.bodies.get_mut(body.rigid_body_handle);
                if let Some(rigid_body) = rigid_body {
                    rigid_body.apply_force(0, &force, ForceType::Impulse, true);
                }
            }

            let collider = physics_world.colliders.get_mut(body.collider_handle);

            if let Some(collider) = collider {
                let material = collider.material_mut().downcast_mut::<BasicMaterial<f32>>();

                if let Some(material) = material {
                    material.friction = friction;
                }
            }
        }
    }
}

pub struct PhysicsSystem;

impl<'a> System<'a> for PhysicsSystem {
    type SystemData = (
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Body>,
        Write<'a, PhysicsWorld>,
        WriteStorage<'a, Animation>,
        WriteStorage<'a, FeetSensor>,
    );

    fn run(
        &mut self,
        (
            mut transform_storage,
            body_storage,
            mut physics_world,
            mut animation_storage,
            mut feet_sensor_storage,
        ): Self::SystemData,
    ) {
        physics_world.step();

        for (transform, body, animation, feet_sensor) in (
            &mut transform_storage,
            &body_storage,
            (&mut animation_storage).maybe(),
            (&mut feet_sensor_storage).maybe(),
        )
            .join()
        {
            let rigid_body = physics_world.bodies.get(body.rigid_body_handle);
            if let Some(rigid_body) = rigid_body {
                let part = rigid_body.part(0).unwrap();
                let point = nphysics2d::nalgebra::Point2::new(0.0, 0.0);
                let iso: Isometry2<f32> = rigid_body.position_at_material_point(part, &point);
                transform.position.x = iso.translation.x;
                transform.position.y = iso.translation.y;

                transform.rotation = iso.rotation.angle();

                let velocity = rigid_body.velocity_at_point(0, &point);

                if let Some(animation) = animation {
                    let on_floor = match feet_sensor {
                        Some(feet) => {
                            let mut feet_grounded = false;
                            let contacts =
                                physics_world.geometrical_world.colliders_interacting_with(
                                    &physics_world.colliders,
                                    feet.collider_handle,
                                );

                            for (_, collider) in contacts.unwrap() {
                                if let Some(user_data) = collider.user_data() {
                                    feet_grounded = match user_data.downcast_ref::<&str>() {
                                        Some(&"brick") => true,
                                        _ => false,
                                    };

                                    if feet_grounded {
                                        break;
                                    }
                                }
                            }

                            feet.on_floor = feet_grounded;

                            feet_grounded
                        }
                        None => false,
                    };

                    let drag_threshold = 10.0;

                    if on_floor {
                        if (velocity.linear.x < -drag_threshold && transform.scale.x > 0.0)
                            || (velocity.linear.x > drag_threshold && transform.scale.x < 0.0)
                        {
                            animation.current_state = AnimationStates::Drag;
                        } else if velocity.linear.x.abs() > 20.0 {
                            animation.current_state = AnimationStates::Moving;
                            animation.speed = animation.speed_factor * velocity.linear.x.abs();
                        } else {
                            animation.current_state = AnimationStates::Idle;
                        }
                    }
                }
            }
        }
    }
}

pub struct CameraSystem;

impl<'a> System<'a> for CameraSystem {
    type SystemData = (
        ReadStorage<'a, Transform>,
        Write<'a, Camera>,
        ReadStorage<'a, CameraTarget>,
    );

    fn run(&mut self, (transform_storage, mut camera, camera_target_flag): Self::SystemData) {
        for (transform, _) in (&transform_storage, &camera_target_flag).join() {
            camera.set_target(&transform.position);
        }
    }
}

pub struct AnimationSystem;

impl<'a> System<'a> for AnimationSystem {
    type SystemData = (WriteStorage<'a, Animation>, WriteStorage<'a, Sprite>);

    fn run(&mut self, (mut animation_storage, mut sprite_storage): Self::SystemData) {
        for (animation, sprite) in (&mut animation_storage, &mut sprite_storage).join() {
            let current_state = animation.animations.get_mut(&animation.current_state);

            if let Some(animation_params) = current_state {
                animation_params.frame += (animation.speed * 1.0 / TARGET_FPS as f32) as f32;
                sprite.src.x = sprite.src.w
                    * ((animation_params.frame.floor() % animation_params.frame_count as f32)
                    + animation_params.start_frame as f32);

                if animation_params.frame >= (animation_params.frame_count + animation_params.start_frame) as f32 {
                    animation_params.frame = animation_params.start_frame as f32;
                }
            }
        }
    }
}

pub struct EnemySystem;

impl<'a> System<'a> for EnemySystem {
    type SystemData = (ReadStorage<'a, BasicAI>, ReadStorage<'a, FeetSensor>, WriteStorage<'a, Body>, Write<'a, PhysicsWorld>);

    fn run(&mut self, (basic_ai_storage, feet_sensor_storage, mut body_storage, mut physics_world): Self::SystemData) {
        let center_point = nphysics2d::nalgebra::Point2::new(0.0, 0.0);
        let mut force = Force2::linear(nphysics2d::nalgebra::Vector2::new(0.0, 0.0));
        let mut top_speed = 50.0;
        let accel = (10000.0 * 1.0 / TARGET_FPS as f32) as f32;


        for (feet_sensor, _, body) in (&feet_sensor_storage, &basic_ai_storage, &mut body_storage).join() {

            let mut rigid_body = physics_world.bodies.get_mut(body.rigid_body_handle);
            if let Some(rigid_body) = rigid_body {
                let current_velocity = rigid_body.velocity_at_point(0, &center_point);

                if current_velocity.linear.x < top_speed {
                    force.linear.x = accel;
                }

                rigid_body.apply_force(0, &force, ForceType::Impulse, true);
            }
        }
    }
}
