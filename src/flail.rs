use rapier2d::prelude::{RigidBodyPosition, RigidBodyBuilder, ColliderBuilder, Collider, Ball, ColliderSet, vector, point, RigidBodySet, RigidBody, RigidBodyHandle, PrismaticJointBuilder, MultibodyJoint, RopeJoint, RevoluteJointBuilder, RopeJointBuilder};
use nalgebra::{Point2};



pub struct Flail<'a> {
    position : Point2<f32>,
    player_position: &'a Point2<f32>,
    handle : RigidBodyHandle,
    joint : RopeJoint,
}

impl Flail <'_> {
    pub fn new<'a>(player_position : &'a Point2<f32>, handle : RigidBodyHandle) -> Flail <'a>{
        let position = *player_position;
        let rigid_body = RigidBodyBuilder::fixed().translation(vector![0.0, 0.0]).build();
        let collider = ColliderBuilder::ball(1.0).build();
        let joint = RopeJointBuilder::new().local_anchor1(point![0.0, 0.0]).limits([0.0,1.0]).build();

        Flail {
            position,
            player_position,
            joint,
            handle,
        }

    }


    pub fn update_position(&mut self, time_delta : f32) {


    }
}
