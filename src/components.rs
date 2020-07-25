use ggez::{
    graphics,
    nalgebra::{Point2, Vector2},
};
use specs::Component;
use specs::{NullStorage, VecStorage};

pub trait Body {
    fn get_bounding_box(&self, transform: &Transform) -> graphics::Rect;
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

#[derive(Clone, Copy, Debug)]
pub struct DynamicBody {
    pub velocity: Vector2<f32>,
    pub width: f32,
    pub height: f32,
}

impl Component for DynamicBody {
    type Storage = VecStorage<Self>;
}

impl Body for DynamicBody {
    fn get_bounding_box(&self, transform: &Transform) -> graphics::Rect {
        let (width, height) = (
            self.width * transform.scale.x,
            self.height * transform.scale.y,
        );

        graphics::Rect::new(
            transform.position.x - width / 2.0,
            transform.position.y - height,
            width,
            height,
        )
    }
}

#[derive(Clone, Copy, Debug)]
pub struct StaticBody {
    pub width: f32,
    pub height: f32,
}

impl Component for StaticBody {
    type Storage = VecStorage<Self>;
}

impl Body for StaticBody {
    fn get_bounding_box(&self, transform: &Transform) -> graphics::Rect {
        let (width, height) = (
            self.width * transform.scale.x,
            self.height * transform.scale.y,
        );
        graphics::Rect::new(
            transform.position.x - width / 2.0,
            transform.position.y - height,
            width,
            height,
        )
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
