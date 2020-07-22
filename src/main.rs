use engine::game::SuperMario;
use ggez::{
    conf::{WindowMode, WindowSetup},
    event, ContextBuilder,
};
use std::{env, path};

mod components;
mod engine;
mod entities;
mod systems;
mod util;



fn main() {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let (mut ctx, mut event_loop) = ContextBuilder::new("super_mario", "Michael Iskandarani")
        .window_setup(WindowSetup::default().title("Super Mario"))
        .window_mode(WindowMode::default().dimensions(1280f32, 720f32))
        .add_resource_path(resource_dir)
        .build()
        .expect("Failed to initialize game!");

    let mut super_mario = SuperMario::new(&mut ctx).unwrap();

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut super_mario) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}
