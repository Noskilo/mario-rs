use crate::components::{Animation, AnimationStates, Sprite};
use crate::components::{Body, CameraTarget, Player, Transform};
use crate::engine::{
    camera::Camera,
    game::TARGET_FPS,
    physics::PhysicsWorld,
    resources::{DebugRenderables, DeltaTime, InputEvents, Renderables},
};
use ggez::{
    event::KeyCode,
    graphics,
    graphics::MeshBuilder,
    nalgebra::{Point2, Vector2},
    Context,
};
use nphysics2d::algebra::{Force2, ForceType, Velocity2};
use specs::prelude::*;
use specs::{world::Index, Read, ReadStorage, System, Write, WriteStorage};
use std::f32::consts::PI;

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
            let draw_param = graphics::DrawParam::new()
                .src(sprite.src)
                .scale(Vector2::new(transform.scale.x, transform.scale.y))
                .dest(Point2::new(
                    (transform.position.x - camera.position.x) + width / (2.0 * camera.zoom),
                    (-transform.position.y + camera.position.y) + height / (2.0 * camera.zoom),
                ))
                .offset(Point2::new(0.5, 0.5))
                .rotation(transform.rotation);

            renderables.0.push_back(draw_param);
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
    );

    fn run(
        &mut self,
        (mut transform_storage, input_events, player, body_storage, mut physics_world): Self::SystemData,
    ) {
        let speed = (300.0 * 1.0 / TARGET_FPS as f32) as f32;
        let mut force = Force2::linear(nphysics2d::nalgebra::Vector2::new(0.0, 0.0));
        for (transform, body, _) in (&mut transform_storage, &body_storage, &player).join() {
            let mut rigid_body = physics_world.bodies.get_mut(body.rigid_body_handle);
            if let Some(rigid_body) = rigid_body {
                if input_events.pressed_keys.contains(&KeyCode::D)
                    || input_events.pressed_keys.contains(&KeyCode::Right)
                {
                    // rigid_body.apply_displacement(&[speed, 0.0, 0.0]);
                    force.linear.x = speed;
                    transform.scale.x = transform.scale.x.abs();
                } else if input_events.pressed_keys.contains(&KeyCode::A)
                    || input_events.pressed_keys.contains(&KeyCode::Left)
                {
                    // rigid_body.apply_displacement(&[-speed, 0.0, 0.0]);
                    force.linear.x = -speed;
                    transform.scale.x = -transform.scale.x.abs();
                } else {
                    force.linear.x = 0.0;
                }

                if input_events.pressed_keys.contains(&KeyCode::Space)
                    && !input_events.repeated_keys.contains(&KeyCode::Space)
                {
                    force.linear.y = 180.0;
                }

                rigid_body.apply_force(0, &force, ForceType::VelocityChange, false);
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
    );

    fn run(
        &mut self,
        (mut transform_storage, body_storage, mut physics_world, mut animation_storage): Self::SystemData,
    ) {
        physics_world.step();

        for (transform, body, animation) in (
            &mut transform_storage,
            &body_storage,
            (&mut animation_storage).maybe(),
        )
            .join()
        {
            let rigid_body = physics_world.bodies.get(body.rigid_body_handle);
            if let Some(rigid_body) = rigid_body {
                let part = rigid_body.part(0).unwrap();
                let point = nphysics2d::nalgebra::Point2::new(0.0, 0.0);
                let iso = rigid_body.position_at_material_point(part, &point);
                transform.position.x = iso.translation.x;
                transform.position.y = iso.translation.y;

                let velocity = rigid_body.velocity_at_point(0, &point);

                if let Some(animation) = animation {
                    if false {
                        animation.current_state = AnimationStates::Jumping;
                    } else if (velocity.linear.x < 0.0 && transform.scale.x > 0.0)
                        || (velocity.linear.x > 0.0 && transform.scale.x < 0.0)
                    {
                        animation.current_state = AnimationStates::Drag;
                    } else if velocity.linear.x.abs() > 20.0 {
                        animation.current_state = AnimationStates::Moving;
                        animation.speed = 0.1 * velocity.linear.x.abs();
                    } else {
                        animation.current_state = AnimationStates::Idle;
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

                if animation_params.frame.floor() > animation_params.frame_count as f32 {
                    animation_params.frame = animation_params.start_frame as f32;
                }
            }
        }
    }
}
