use nphysics2d::{
    force_generator::DefaultForceGeneratorSet,
    joint::DefaultJointConstraintSet,
    nalgebra::Vector2,
    object::{DefaultBodySet, DefaultColliderSet, RigidBody},
    world::{DefaultGeometricalWorld, DefaultMechanicalWorld},
    ncollide2d::shape::{ShapeHandle, Ball},
};
use nphysics2d::object::{RigidBodyDesc, DefaultBodyHandle, ColliderDesc, BodyPartHandle, Collider};
use crate::components::Body;

pub struct PhysicsWorld {
    mechanical_world: DefaultMechanicalWorld<f32>,
    geometrical_world: DefaultGeometricalWorld<f32>,
    colliders: DefaultColliderSet<f32>,
    pub(crate) bodies: DefaultBodySet<f32>,
    joint_constraints: DefaultJointConstraintSet<f32>,
    force_generators: DefaultForceGeneratorSet<f32>,
}

impl PhysicsWorld {
    pub fn step(&mut self) {
        self.mechanical_world.step(
            &mut self.geometrical_world,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.joint_constraints,
            &mut self.force_generators,
        );
    }

    pub fn insert_body(&mut self, rigid_body: RigidBody<f32>, collider_desc: ColliderDesc<f32>) -> Body {
        let parent_handle = self.bodies.insert(rigid_body);

        let collider = collider_desc
            .build(BodyPartHandle(parent_handle, 0));
        let collider_handle = self.colliders.insert(collider);
        Body {
            rigid_body_handle: parent_handle,
            collider_handle,
        }
    }
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        let mechanical_world = DefaultMechanicalWorld::new(Vector2::new(0.0, -300.0));
        let geometrical_world = DefaultGeometricalWorld::new();

        let bodies = DefaultBodySet::new();
        let colliders = DefaultColliderSet::new();
        let joint_constraints = DefaultJointConstraintSet::new();
        let force_generators = DefaultForceGeneratorSet::new();

        Self {
            mechanical_world,
            geometrical_world,
            bodies,
            colliders,
            joint_constraints,
            force_generators,
        }
    }
}
