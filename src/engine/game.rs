use crate::components::Transform;
use crate::components::{RigidBody, Visual};
use crate::engine::global_resources::DeltaTime;
use crate::engine::global_resources::Renderable;
use crate::engine::resource_manager::ResourceManager;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;

use crate::engine::resource_manager::TextureManager;
use crate::{
    engine::global_resources::InputEvents,
    systems::{Physics, PlayerControlSystem, RenderObjects},
};
use legion::prelude::{Resources, Schedule, Universe};
use sdl2::render::{Canvas, Texture};

use nalgebra::Vector2;
use sdl2::EventPump;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect, video::Window};
use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

const TARGET_FPS: u128 = 60;

pub struct Game {
    pub is_running: bool,
    event_pump: EventPump,
    resources: Resources,
    canvas: Canvas<Window>,
    universe: Universe,
}

impl<'a> Game {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("rust-sdl2 demo", 800, 600)
            .position_centered()
            .resizable()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();
        let mut resources = Resources::default();

        resources.insert(DeltaTime(0.0));
        resources.insert(InputEvents(Vec::new(), Vec::new()));
        resources.insert(Renderable(VecDeque::new()));

        let universe = Universe::new();

        Self {
            is_running: false,
            resources,
            event_pump,
            canvas,
            universe,
        }
    }

    fn queue_events(&mut self) {
        let mut input_events = self.resources.get_mut::<InputEvents>().unwrap();

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.is_running = false,
                event => {
                    input_events.0.push(event);
                }
            }
        }

        for keycode in self
            .event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
        {
            input_events.1.push(keycode);
        }
    }

    fn flush_events(&mut self) {
        let mut input_events = self.resources.get_mut::<InputEvents>().unwrap();
        input_events.0.clear();
        input_events.1.clear();
    }

    fn update(&mut self, delta_time: f64) {
        let mut delta_resource = self.resources.get_mut::<DeltaTime>().unwrap();
        delta_resource.0 = delta_time;
    }

    fn render(
        &mut self,
        texture_manager: &mut ResourceManager<
            'a,
            String,
            Texture<'a>,
            TextureCreator<WindowContext>,
        >,
    ) {
        let mut renderables = self.resources.get_mut::<Renderable>().unwrap();
        self.canvas.set_draw_color(Color::RGB(32, 64, 255));
        self.canvas.clear();

        while !renderables.0.is_empty() {
            let renderable = renderables.0.pop_front().unwrap();

            let texture = texture_manager.load(&renderable.2).unwrap();

            let transform = renderable.0;

            self.canvas
                .copy_ex(
                    &texture,
                    renderable.1,
                    Rect::new(
                        transform.position.x as i32,
                        transform.position.y as i32,
                        (renderable.1.width() as f32 * transform.scale) as u32,
                        (renderable.1.height() as f32 * transform.scale) as u32,
                    ),
                    transform.rotation as f64,
                    None,
                    false,
                    false,
                )
                .unwrap();
        }

        self.canvas.present();
    }

    pub fn run(mut self) {
        self.is_running = true;

        let mut frame_timer;
        let mut delta_time = 0.0;

        let mut world = self.universe.create_world();

        let input_system = PlayerControlSystem::build();
        let render_objects = RenderObjects::build();
        let physics = Physics::build();

        let mut schedule = Schedule::builder()
            .add_system(input_system)
            .add_system(render_objects)
            .add_system(physics)
            .flush()
            .build();

        let texture_creator = self.canvas.texture_creator();
        let mut texture_manager = TextureManager::new(&texture_creator);

        let mario = "assets/textures/mario.png";
        texture_manager.load(mario).unwrap();

        let components = vec![(
            Transform {
                position: Vector2::new(0f32, 0f32),
                rotation: 0.0,
                scale: 2.0,
            },
            Visual {
                texture_id: mario.to_string(),
                src_rect: Rect::new(32, 0, 32, 32),
            },
            RigidBody {
                velocity: Vector2::new(0f32, 0f32),
                bounding_box: Rect::new(0, 0, 32, 32),
            },
        )];

        world.insert((), components);

        let target_time: u128 = 1_000_000_000 / TARGET_FPS;

        while self.is_running {
            frame_timer = Instant::now();

            self.queue_events();

            self.update(delta_time);
            schedule.execute(&mut world, &mut self.resources);

            self.render(&mut texture_manager);

            self.flush_events();

            let frame_time = frame_timer.elapsed().as_nanos();

            if target_time > frame_time {
                ::std::thread::sleep(Duration::from_nanos((target_time - frame_time) as u64));
            }
            delta_time = frame_timer.elapsed().as_secs_f64();
        }
    }
}
