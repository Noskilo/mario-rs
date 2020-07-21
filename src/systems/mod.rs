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
