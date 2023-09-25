use rapier2d::prelude::{RigidBodyPosition, RigidBodyBuilder, ColliderBuilder, Collider, Ball, ColliderSet, vector, point, RigidBodySet, RigidBody, RigidBodyHandle, PrismaticJointBuilder, MultibodyJoint, RopeJoint, RevoluteJointBuilder, RopeJointBuilder};
//use nalgebra::{Vector2};
use glam::Vec2;



pub struct Flail {
    pub position : Vec2,
    length : f32,
    //body_handle : RigidBodyHandle,
}

impl Flail {
    pub fn new(position: Vec2) -> Flail {
        //let position = *player_position;
        Flail {
            position,
            length : 5.0,
        }
    }

    pub fn update_position(&mut self, player_position : Vec2, time_delta : f64) {
        //let body = RigidBodySet::get_mut(RigidBodyHandle).unwrap();
        let direction : Vec2 = Vec2::normalize_or_zero(self.position - player_position);
        let distance = Vec2::distance(self.position, player_position);
        let force : f32 = (distance-self.length)*(distance-self.length);
        let time_delta = time_delta as f32*1.0;
        self.position = self.position + direction*force*time_delta;
    }
}
