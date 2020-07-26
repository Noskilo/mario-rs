use ggez::{
    graphics,
    nalgebra::{Point2},
};
use nphysics2d::nalgebra::base::Vector2;
use specs::Component;
use specs::{NullStorage, VecStorage};

use nphysics2d::object::{DefaultBodyHandle, DefaultColliderHandle};
use std::collections::HashMap;


#[derive(Clone, Copy, Debug)]
pub struct Body {
    pub rigid_body_handle: DefaultBodyHandle,
    pub collider_handle: DefaultColliderHandle,
}

impl Component for Body {
    type Storage = VecStorage<Self>;
}

#[derive(Clone, Copy, Debug)]
pub struct FeetSensor {
    pub collider_handle: DefaultColliderHandle,
    pub on_floor: bool
}

impl Component for FeetSensor {
    type Storage = VecStorage<Self>;
}

#[derive(Clone, Copy, Debug)]
pub struct Transform {
    pub position: Point2<f32>,
    pub rotation: f32,
    pub scale: Vector2<f32>,
}

impl Component for Transform {
    type Storage = VecStorage<Self>;
}

#[derive(Clone, Copy, Debug)]
pub struct Sprite {
    pub src: graphics::Rect,
    pub width: f32,
    pub height: f32,
}

impl Component for Sprite {
    type Storage = VecStorage<Self>;
}

#[derive(Clone, Debug, Default)]
pub struct Animation {
    pub current_state: AnimationStates,
    pub animations: HashMap<AnimationStates, AnimationParams>,
    pub speed: f32,
}

impl Component for Animation {
    type Storage = VecStorage<Self>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct AnimationParams {
    pub frame: f32,
    pub start_frame: u32,
    pub frame_count: u32,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum AnimationStates {
    Idle,
    Moving,
    Jumping,
    Drag
}

impl Default for AnimationStates {
    fn default() -> Self {
        AnimationStates::Idle
    }
}


#[derive(Clone, Copy, Debug, Default)]
pub struct CameraTarget;

impl Component for CameraTarget {
    type Storage = NullStorage<Self>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Player;

impl Component for Player {
    type Storage = NullStorage<Self>;
}
