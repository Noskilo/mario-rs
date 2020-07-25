// pub struct PlayerControlSystem;

// impl PlayerControlSystem {
//     pub fn build() -> Box<dyn Schedulable> {
//         SystemBuilder::new("update_positions")
//             .read_resource::<InputEvents>()
//             .read_resource::<DeltaTime>()
//             .with_query(<Write<RigidBody>>::query())
//             .build(|_, world, (input_events, delta_time), query| {
//                 let right = Keycode::D;
//                 let left = Keycode::A;

//                 let mut movement = Vector2::<f32>::new(0.0, 0.0);

//                 let speed = (220.0 * delta_time.0) as f32;
//                 let jump_force = (300.0 * delta_time.0) as f32;

//                 if input_events.1.contains(&right) {
//                     movement.x += speed;
//                 } else if input_events.1.contains(&left) {
//                     movement.x -= speed;
//                 }

//                 for event in input_events.0.iter() {
//                     match event {
//                         Event::KeyDown {
//                             keycode: Some(Keycode::Space),
//                             repeat: false,
//                             ..
//                         } => {
//                             movement.y -= jump_force;
//                         }
//                         _ => (),
//                     }
//                 }

//                 // if input_events.1.contains(&up) {
//                 //     movement.y -= speed;
//                 // } else if input_events.1.contains(&down) {
//                 //     movement.y += speed;
//                 // }

//                 if movement.magnitude() != 0f32 {
//                     for mut body in query.iter_mut(world) {
//                         body.velocity.x = movement.x;
//                         if movement.y < 0f32 {
//                             body.velocity.y = movement.y;
//                         }
//                     }
//                 }
//             })
//     }
// }

// pub struct RenderObjects;

// impl RenderObjects {
//     pub fn build() -> Box<dyn Schedulable> {
//         SystemBuilder::new("render_objects")
//             .write_resource::<Renderable>()
//             .with_query(<(Read<Transform>, Read<Visual>)>::query())
//             .build(|_, world, render_queue, query| {
//                 for (trans, vis) in query.iter(world) {
//                     render_queue
//                         .0
//                         .push_back((*trans, vis.src_rect, vis.texture_id.clone()));
//                 }
//             })
//     }
// }

// pub struct Physics;

// impl Physics {
//     pub fn build() -> Box<dyn Schedulable> {
//         SystemBuilder::new("physics")
//             .read_resource::<DeltaTime>()
//             .with_query(<(Write<Transform>, Write<RigidBody>)>::query())
//             .build(|_, world, delta_time, query| {
//                 for (mut trans, mut body) in query.iter_mut(&mut *world) {
//                     body.velocity.y += 10f32 * delta_time.0 as f32;
//                     trans.position += body.velocity;
//                 }
//             })
//     }

//     fn aabb(box1: &Rect, box2: &Rect) -> bool {
//         box1.has_intersection(*box2)
//     }
// }

use crate::components::Sprite;
use crate::components::{Body, CameraTarget, DynamicBody, Player, StaticBody, Transform};
use crate::engine::{
    camera::Camera,
    game::TARGET_FPS,
    resources::{DebugRenderables, DeltaTime, InputEvents, Renderables},
};
use ggez::{event::KeyCode, graphics, graphics::MeshBuilder, nalgebra::Point2, Context};
use specs::prelude::*;
use specs::{world::Index, Read, ReadStorage, System, Write, WriteStorage};

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
        ReadStorage<'a, DynamicBody>,
        ReadStorage<'a, StaticBody>,
        Write<'a, Renderables>,
        Write<'a, DebugRenderables>,
        Read<'a, Camera>,
    );

    fn run(
        &mut self,
        (
            sprite_storage,
            transform_storage,
            dynamic_body_storage,
            static_body_storage,
            mut renderables,
            mut debug_renderables,
            camera,
        ): Self::SystemData,
    ) {
        let (width, height) = graphics::size(self.ctx);

        for (sprite, transform) in (&sprite_storage, &transform_storage).join() {
            let draw_param = graphics::DrawParam::new()
                .src(sprite.src)
                .scale(transform.scale)
                .dest(Point2::new(
                    (transform.position.x - camera.position.x - sprite.width / 2.0)
                        + width / (2.0 * camera.zoom),
                    (transform.position.y - camera.position.y - sprite.height / 2.0)
                        + height / (2.0 * camera.zoom),
                ));

            renderables.0.push_back(draw_param);
        }

        // for (transform, dynamic_body) in (&transform_storage, &dynamic_body_storage).join() {
        //     let debug_rect = graphics::Rect::new(
        //         (transform.position.x - camera.position.x - dynamic_body.width / 2.0)
        //             + width / (2.0 * camera.zoom),
        //         (transform.position.y - camera.position.y - dynamic_body.height / 2.0)
        //             + height / (2.0 * camera.zoom),
        //         dynamic_body.width,
        //         dynamic_body.height,
        //     );

        //     let mesh = MeshBuilder::new()
        //         .rectangle(
        //             ggez::graphics::DrawMode::stroke(0.5),
        //             debug_rect,
        //             ggez::graphics::WHITE,
        //         )
        //         .build(self.ctx)
        //         .unwrap();

        //     debug_renderables.0.push_back(mesh);
        // }

        // for (transform, static_body) in (&transform_storage, &static_body_storage).join() {
        //     let debug_rect = graphics::Rect::new(
        //         (transform.position.x - camera.position.x - static_body.width / 2.0)
        //             + width / (2.0 * camera.zoom),
        //         (transform.position.y - camera.position.y - static_body.height / 2.0)
        //             + height / (2.0 * camera.zoom),
        //         static_body.width,
        //         static_body.height,
        //     );

        //     let mesh = MeshBuilder::new()
        //         .rectangle(
        //             ggez::graphics::DrawMode::stroke(0.5),
        //             debug_rect,
        //             ggez::graphics::BLACK,
        //         )
        //         .build(self.ctx)
        //         .unwrap();

        //     debug_renderables.0.push_back(mesh);
        // }
    }
}

pub struct PlayerControlSystem;

impl<'a> System<'a> for PlayerControlSystem {
    type SystemData = (
        WriteStorage<'a, Transform>,
        WriteStorage<'a, DynamicBody>,
        Read<'a, InputEvents>,
        ReadStorage<'a, Player>,
    );

    fn run(
        &mut self,
        (mut transform_storage, mut body_storage, input_events, player): Self::SystemData,
    ) {
        let speed = (100.0 * 1.0 / TARGET_FPS as f32) as f32;
        for (trans, body, _) in (&mut transform_storage, &mut body_storage, &player).join() {
            if input_events.pressed_keys.contains(&KeyCode::D)
                || input_events.pressed_keys.contains(&KeyCode::Right)
            {
                trans.position.x += speed;
            } else if input_events.pressed_keys.contains(&KeyCode::A)
                || input_events.pressed_keys.contains(&KeyCode::Left)
            {
                trans.position.x -= speed;
            }
            if input_events.pressed_keys.contains(&KeyCode::W) {
            } else if input_events.pressed_keys.contains(&KeyCode::S) {
            }

            if input_events.pressed_keys.contains(&KeyCode::Space) {
                body.velocity.y = -5.0;
            }
        }
    }
}

pub struct PhysicsSystem;

impl<'a> System<'a> for PhysicsSystem {
    type SystemData = (
        WriteStorage<'a, Transform>,
        WriteStorage<'a, DynamicBody>,
        ReadStorage<'a, StaticBody>,
    );

    fn run(&mut self, (mut transforms, mut body_storage, static_body_storage): Self::SystemData) {
        let mut boxes = Vec::new();
        for (transform, body) in (&transforms, &static_body_storage).join() {
            let bounding_box = body.get_bounding_box(transform);

            boxes.push(bounding_box);
        }

        for (transform, body) in (&mut transforms, &mut body_storage).join() {
            body.velocity.y += 0.2;

            transform.position += body.velocity;

            let dynamic_bounding_box = body.get_bounding_box(transform);

            for bounding_box in boxes.iter() {
                if dynamic_bounding_box.overlaps(bounding_box) {
                    body.velocity.y = 0.0;

                    transform.position.y = bounding_box.top();
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
            camera.set_target(transform.position);
        }
    }
}
