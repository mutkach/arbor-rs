use rapier2d::prelude::{vector, RigidBodySet, RigidBody, Ball, ColliderSet, QueryPipeline, QueryFilter, RigidBodyBuilder};
use rapier2d::control::KinematicCharacterController;
use rapier2d::math::{Real, Vector};

pub struct Player {
    pub position : Vector<Real>,
}

impl Player {
    pub fn new(position : Vector<Real>) -> Player {
        Player {
            position,
        }
    }

    pub fn move_player(&mut self, desired_movement : Vector<Real>, time_delta : f64) {
        let direction : Vector<Real> = desired_movement;
        let time_delta = time_delta as f32;
        self.position = self.position + direction*time_delta*50.0;
    }
}
