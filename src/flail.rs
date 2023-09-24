use rapier2d::prelude::{RigidBodyPosition, RigidBodyBuilder};


pub struct Flail<'a> {
    position: Point2<f32>,
    player_position: &'a Point2<f32>,
    handle: RigidBodyHandle,
    joint: RopeJoint,
}

impl Flail <'_> {
    pub fn new<'a>(player_position: &'a Point2<f32>) -> Flail <'a> {
        let position = *player_position;
        
    }
}
