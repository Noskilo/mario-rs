use engine::Game;

mod engine;
mod systems;
mod components;

fn main() {
    let game = Game::new();

    game.run();
}
