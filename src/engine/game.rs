use super::scene_manager::Scene;
use crate::{
    components::{Sprite, Transform, DynamicBody, StaticBody, CameraTarget, Player},
    engine::scene_manager::SceneManager,
    entities::{Brick, Mario},
    systems::{PlayerControlSystem, PhysicsSystem, CameraSystem},
};
use ggez::{
    event::EventHandler,
    graphics,
    nalgebra::{Point2, Vector2},
    timer, Context, GameResult,
};
use graphics::{FilterMode, Rect};
use specs::{Builder, DispatcherBuilder, World, WorldExt};

pub const TARGET_FPS: u32 = 60;

pub struct SuperMario<'a, 'b> {
    pub is_running: bool,
    scene_manager: SceneManager<'a, 'b>,
    batch: graphics::spritebatch::SpriteBatch,
}

impl<'a, 'b> SuperMario<'a, 'b> {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut world = World::new();
        world.register::<Transform>();
        world.register::<Sprite>();
        world.register::<DynamicBody>();
        world.register::<StaticBody>();
        world.register::<CameraTarget>();
        world.register::<Player>();

        Mario::add(&mut world, Point2::new(0.0, 0.0));

        for i in 0..100 {
            Brick::add(&mut world, Point2::new(i as f32 * 16f32, 30.0));
        }
    

        let dispatcher = DispatcherBuilder::new()
            .with(PlayerControlSystem, "PlayerControlSystem", &[])
            .with(PhysicsSystem, "PhysicsSystem", &[])
            .with(CameraSystem, "CameraSystem", &[])
            .build();
        let first_scene = Scene::new(world, dispatcher);

        let image = graphics::Image::new(ctx, "/textures.png").unwrap();

        let mut batch = graphics::spritebatch::SpriteBatch::new(image);
        batch.set_filter(FilterMode::Nearest);

        let game = Self {
            is_running: false,
            scene_manager: SceneManager::new(first_scene),
            batch,
        };

        Ok(game)
    }
}

impl<'a, 'b> EventHandler for SuperMario<'a, 'b> {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while timer::check_update_time(ctx, TARGET_FPS) {
            self.scene_manager.update(ctx)?;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::Color::new(0.2, 0.3, 0.5, 1.0));

        self.scene_manager.draw(ctx, &mut self.batch)?;

        self.batch.clear();
        graphics::present(ctx)?;

        Ok(())
    }
}
