use std::collections::{HashMap, HashSet, vec_deque::VecDeque};
use std::time::{Duration, Instant};

use ggez::{
    event::{KeyCode, KeyMods},
    graphics::{DrawParam, Mesh},
};

#[derive(Default)]
pub struct DeltaTime(pub f64);

#[derive(Default)]
pub struct Renderables(pub VecDeque<DrawParam>);

#[derive(Default)]
pub struct DebugRenderables(pub VecDeque<Mesh>);

#[derive(Default)]
pub struct InputEvents {
    pub pressed_keys: HashMap<KeyCode, Instant>,
    pub active_mods: KeyMods,
}

impl InputEvents {
    pub fn is_mod_active(&self, keymods: KeyMods) -> bool {
        self.active_mods.contains(keymods)
    }

    pub fn is_key_pressed(&self, key: &KeyCode) -> bool {
        self.pressed_keys.contains_key(key)
    }

    pub fn key_hold_time(&self, key: &KeyCode) -> Option<&Instant> {
        self.pressed_keys.get(key)
    }

    pub fn is_key_just_pressed(&self, key: &KeyCode) -> bool {
        self.is_key_pressed(key) && self.key_hold_time(key).unwrap().elapsed().as_millis() < 100
    }
}
