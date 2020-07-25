use super::{
    camera::Camera,
    resources::{DeltaTime, InputEvents, Renderables, DebugRenderables},
};
use ggez::error::GameResult;

use crate::systems::RenderingSystem;
use ggez::{
    event::KeyCode,
    graphics::{spritebatch::SpriteBatch, MeshBuilder, self},
    input::keyboard,
    timer, Context, nalgebra::Point2,
};
use specs::Dispatcher;
use specs::{RunNow, World, WorldExt};
use graphics::DrawParam;

pub struct SceneManager<'a, 'b> {
    scenes: Vec<Box<Scene<'a, 'b>>>,
}

impl<'a, 'b> SceneManager<'a, 'b> {
    pub fn new(first_scene: Scene<'a, 'b>) -> Self {
        Self {
            scenes: vec![Box::new(first_scene)],
        }
    }

    pub fn current_scene(&mut self) -> &mut Scene<'a, 'b> {
        self.scenes.first_mut().unwrap()
    }

    pub fn push_scene(&mut self, scene: Scene<'a, 'b>) {
        self.scenes.push(Box::new(scene));
    }

    pub fn pop_scene(&'a mut self) -> Option<Box<Scene<'a, 'b>>> {
        self.scenes.pop()
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.current_scene().update(ctx)
    }

    pub fn draw(&mut self, ctx: &mut Context, batch: &mut SpriteBatch) -> GameResult<()> {
        self.current_scene().draw(ctx, batch)
    }
}

pub struct Scene<'a, 'b> {
    dispatcher: Dispatcher<'a, 'b>,
    world: World,
}

impl<'a, 'b> Scene<'a, 'b> {
    pub fn new(mut world: World, mut dispatcher: Dispatcher<'a, 'b>) -> Self {
        world.insert(DeltaTime(0.0));
        world.insert(Renderables::default());
        world.insert(Camera::default());
        world.insert(InputEvents::default());
        world.insert(DebugRenderables::default());

        dispatcher.setup(&mut world);
        Self { world, dispatcher }
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        {
            let mut input_events = self.world.write_resource::<InputEvents>();
            let mut cam = self.world.write_resource::<Camera>();
            let mut delta = self.world.write_resource::<DeltaTime>();
            delta.0 = timer::delta(ctx).as_secs_f64();

            cam.update();

            input_events.active_mods = keyboard::active_mods(ctx);
            input_events.pressed_keys = keyboard::pressed_keys(ctx).clone();
        }

        self.dispatcher.dispatch(&self.world);
        self.world.maintain();
        Ok(())
    }

    fn draw(&self, ctx: &mut Context, batch: &mut SpriteBatch) -> GameResult<()> {
        {
            let mut render_system = RenderingSystem::new(ctx);

            render_system.run_now(&self.world);
        }


        let mut renderables = self.world.write_resource::<Renderables>();
        let mut debug_renderables = self.world.write_resource::<DebugRenderables>();
        let cam = self.world.read_resource::<Camera>();

        while !renderables.0.is_empty() {
            let renderable = renderables.0.pop_front().unwrap();
            batch.add(renderable);
        }

        cam.render(ctx, batch)?;
        cam.debug_render(ctx, &mut debug_renderables.0)?;



        Ok(())
    }
}
