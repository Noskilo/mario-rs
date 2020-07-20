use sdl2::keyboard::Keycode;
use crate::components::Transform;
use sdl2::{event::Event, rect::Rect};
use std::collections::vec_deque::VecDeque;

pub struct DeltaTime(pub f64);

pub struct InputEvents(pub Vec<Event>, pub Vec<Keycode>);

pub struct Renderable(pub VecDeque<(Transform, Rect, String)>);
