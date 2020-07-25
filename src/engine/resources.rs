use ggez::{
    event::{KeyCode, KeyMods},
    graphics::{Mesh, DrawParam},
};
use std::collections::{vec_deque::VecDeque, HashSet};

#[derive(Default)]
pub struct DeltaTime(pub f64);

#[derive(Default)]
pub struct Renderables(pub VecDeque<DrawParam>);

#[derive(Default)]
pub struct DebugRenderables(pub VecDeque<Mesh>);

#[derive(Default)]
pub struct InputEvents {
    pub pressed_keys: HashSet<KeyCode>,
    pub active_mods: KeyMods,
    pub repeated_keys: HashSet<KeyCode>,
}

impl InputEvents {
    pub fn is_mod_active(&self, keymods: KeyMods) -> bool {
        self.active_mods.contains(keymods)
    }

    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.pressed_keys.contains(&key)
    }
}
// pub struct InputEvents(pub Vec<Event>, pub Vec<Keycode>);
