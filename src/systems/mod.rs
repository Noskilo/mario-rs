use crate::components::Transform;
use crate::engine::global_resources::DeltaTime;
use crate::engine::global_resources::Renderable;
use crate::{components::Visual, engine::global_resources::InputEvents};
use legion::prelude::*;
use legion::query::Read;
use legion::{prelude::Schedulable, systems::SystemBuilder};
use nalgebra::Vector2;
use sdl2::keyboard::Keycode;

pub struct PlayerControlSystem;

impl PlayerControlSystem {
    pub fn build() -> Box<dyn Schedulable> {
        SystemBuilder::new("update_positions")
            .read_resource::<InputEvents>()
            .read_resource::<DeltaTime>()
            .with_query(<Write<Transform>>::query())
            .build(|_, world, (input_events, delta_time), query| {
                let right = Keycode::D;
                let up = Keycode::W;
                let down = Keycode::S;
                let left = Keycode::A;

                let mut movement = Vector2::<f32>::new(0.0, 0.0);

                let speed = (220.0 * delta_time.0) as f32;

                if input_events.1.contains(&right) {
                    movement.x += speed;
                } else if input_events.1.contains(&left) {
                    movement.x -= speed;
                }

                if input_events.1.contains(&up) {
                    movement.y -= speed;
                } else if input_events.1.contains(&down) {
                    movement.y += speed;
                }

                if movement.magnitude() != 0f32 {
                    for mut transform in query.iter_mut(world) {
                        transform.position += movement;
                    }
                }
            })
    }
}

pub struct RenderObjects;

impl RenderObjects {
    pub fn build() -> Box<dyn Schedulable> {
        SystemBuilder::new("render_objects")
            .write_resource::<Renderable>()
            .with_query(<(Read<Transform>, Read<Visual>)>::query())
            .build(|_, world, render_queue, query| {
                for (trans, vis) in query.iter(world) {

                    render_queue
                        .0
                        .push_back((*trans, vis.src_rect, vis.texture_id.clone()));
                }
            })
    }
}
