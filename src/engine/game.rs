use ggez::{
    Context,
    event::EventHandler,
    GameResult,
    graphics, nalgebra::{Point2, Vector2}, timer,
};
use graphics::{FilterMode, Rect};
use specs::{Builder, DispatcherBuilder, World, WorldExt};

use crate::{
    components::{Body, CameraTarget, Player, Sprite, Transform},
    engine::scene_manager::SceneManager,
    entities::{Brick, Mario},
    systems::{CameraSystem, PhysicsSystem, PlayerControlSystem},
};
use crate::components::{Animation, FeetSensor, Jumper, BasicAI};
use crate::systems::{AnimationSystem, EnemySystem};

use super::{physics::PhysicsWorld, scene_manager::Scene};
use crate::entities::KoopaTroopa;

pub const TARGET_FPS: u32 = 60;

pub struct SuperMario<'a, 'b> {
    pub is_running: bool,
    scene_manager: SceneManager<'a, 'b>,
    batch: graphics::spritebatch::SpriteBatch,
}

impl<'a, 'b> SuperMario<'a, 'b> {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut physics_world = PhysicsWorld::default();
        let mut world = World::new();
        world.register::<Transform>();
        world.register::<Sprite>();
        world.register::<Body>();
        world.register::<CameraTarget>();
        world.register::<Player>();
        world.register::<Animation>();
        world.register::<FeetSensor>();
        world.register::<Jumper>();
        world.register::<BasicAI>();


        Mario::add(&mut world, Point2::new(0.0, 0.0), &mut physics_world);
        KoopaTroopa::add(&mut world, Point2::new(32.0, 0.0) , &mut physics_world);

        for i in 0..1 {
            Brick::add(&mut world, Point2::new(i as f32 * 7.0 * 16.0, (i + 1) as f32 * -32.0), nphysics2d::nalgebra::base::Vector2::new(30, 2), &mut physics_world);
        }
      

        let dispatcher = DispatcherBuilder::new()
            .with(PlayerControlSystem, "PlayerControlSystem", &[])
            .with(PhysicsSystem, "PhysicsSystem", &["PlayerControlSystem"])
            .with(AnimationSystem, "AnimationSystem", &[])
            .with(CameraSystem, "CameraSystem", &[])
            .with(EnemySystem, "EnemySystem", &[])
            .build();
        let first_scene = Scene::new(world, dispatcher, physics_world);

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
