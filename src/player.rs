use std::ops::AddAssign;

//use rapier2d::prelude::{vector, RigidBodySet, RigidBody, Ball, ColliderSet, QueryPipeline, QueryFilter, RigidBodyBuilder};
//use rapier2d::control::KinematicCharacterController;
//use nalgebra::Vec2;
use glam::Vec2;

pub struct Player {
    pub position : Vec2,
}

impl Player {
    pub fn new(position : Vec2) -> Player {
        Player {
            position,

        }
    }

    pub fn move_player(&mut self, desired_movement : Vec2, time_delta : f64) {
        let direction : Vec2 = Vec2::normalize_or_zero(desired_movement);
        let time_delta = time_delta as f32;
        self.position = self.position + direction*time_delta*10.0;
    }
    //pub fn update_position(&mut self,  {
    //}
}
