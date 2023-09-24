use std::ops::AddAssign;

use rapier2d::prelude::{vector, RigidBodySet, RigidBody, Ball, ColliderSet, QueryPipeline, QueryFilter, RigidBodyBuilder};
use rapier2d::control::{KinematicCharacterController};
use nalgebra::{Vector2};

pub struct Player {
    position : Vector2<f32>,
    shape : Ball,
    handle : RigidBodyHandle,
    controller : KinematicCharacterController,
}

impl Player {
    pub fn new(position : Vector2<f32>, radius : f32) -> Player {
        Player {
            position,
            shape : Ball::new(radius),
            body : RigidBodyBuilder::new().build();
            controller : KinematicCharacterController::default(),

        }
    }

    pub fn move_player(&mut self, desired_movement : Vector2<f32>){
        self.position.add_assign(desired_movement);
    }
}
