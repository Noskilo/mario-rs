use engine::game::SuperMario;
use ggez::{conf::WindowSetup, event, ContextBuilder};
use std::path;

mod components;
mod engine;
mod systems;

fn main() {
    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("super_mario", "Michael Iskandarani")
        .window_setup(WindowSetup::default().title("Super Mario"))
        .add_resource_path(path::PathBuf::from("./assets"))
        .build()
        .expect("Failed to initialize game!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut super_mario = SuperMario::new(&mut ctx).unwrap();

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut super_mario) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}
