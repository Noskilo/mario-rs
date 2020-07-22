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
use crate::components::Transform;
use crate::engine::{
    camera::Camera,
    game::TARGET_FPS,
    resources::{DeltaTime, InputEvents, Renderables},
};
use ggez::{event::KeyCode, graphics, nalgebra::Point2, Context};
use specs::prelude::*;
use specs::{Read, ReadStorage, System, Write, WriteStorage};

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
        Read<'a, Camera>,
    );

    fn run(&mut self, (sprite, transform, mut renderables, camera): Self::SystemData) {
        let (width, height) = graphics::size(self.ctx);

        for (sprite, transform) in (&sprite, &transform).join() {
            let draw_param = graphics::DrawParam::new()
                .src(sprite.src)
                .offset(Point2::new(0.5, 0.5))
                .scale(transform.scale)
                .dest(Point2::new(
                    (transform.position.x - camera.position.x - sprite.width / 2.0)
                        + width / (2.0 * camera.zoom),
                    (transform.position.y - camera.position.y - sprite.height / 2.0)
                        + height / (2.0 * camera.zoom),
                ));

            renderables.0.push_back(draw_param);
        }
    }
}

pub struct PlayerControlSystem;

impl<'a> System<'a> for PlayerControlSystem {
    type SystemData = (
        WriteStorage<'a, Transform>,
        Read<'a, InputEvents>,
        Write<'a, Camera>,
    );

    fn run(&mut self, (mut transform, input_events, mut camera): Self::SystemData) {
        let speed = (100.0 * 1.0 / TARGET_FPS as f32) as f32;
        for trans in (&mut transform).join() {
            if input_events.pressed_keys.contains(&KeyCode::D) {
                // println!("Jump");
                trans.position.x += speed;
            } else if input_events.pressed_keys.contains(&KeyCode::A) {
                trans.position.x -= speed;
            }
            if input_events.pressed_keys.contains(&KeyCode::W) {
                // println!("Jump");
                trans.position.y -= speed;
            } else if input_events.pressed_keys.contains(&KeyCode::S) {
                trans.position.y += speed;
            }

            camera.set_target(trans.position);
        }
    }
}
