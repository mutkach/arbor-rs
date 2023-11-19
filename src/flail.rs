use rapier2d::math::{Real, Vector};
use rapier2d::prelude::{
    vector, ColliderBuilder, ColliderSet, Isometry, RigidBodyBuilder, RigidBodyHandle,
    RigidBodySet, RigidBodyType,
};

pub struct Flail {
    pub position: Vector<Real>,
    length: f32,
    body_handle: RigidBodyHandle,
}

impl Flail {
    pub fn new(
        position: Vector<Real>,
        bodies: &mut RigidBodySet,
        colliders: &mut ColliderSet,
    ) -> Flail {
        let collider = ColliderBuilder::ball(0.5).restitution(0.7).build();
        let body = RigidBodyBuilder::new(RigidBodyType::Dynamic)
            .position(Isometry::new(vector![position.x, position.y], 0.0))
            .additional_mass(10.0)
            .linear_damping(0.1)
            .build();

        let body_handle = bodies.insert(body);
        colliders.insert_with_parent(collider, body_handle, bodies);

        Flail {
            position,
            length: 10.0,
            body_handle,
        }
    }

    pub fn update_position(&mut self, player_position: Vector<Real>, bodies: &mut RigidBodySet) {
        let body = bodies.get_mut(self.body_handle).unwrap();
        let scale: f32 = Real::abs(self.position.metric_distance(&player_position) - self.length);
        let elastic_coef = 1.2;
        let elastic_force = (player_position - self.position) * scale * elastic_coef;
        //let direction = velocity.normalize();
        //let friction_force = -direction*self.mass;
        //let wind_friction_force = direction*velocity*velocity*0.01;
        body.reset_forces(true);
        //if elastic_force.magnitude() > friction_force.magnitude() {
        body.add_force(vector![elastic_force.x, elastic_force.y], true);
        self.position = body.position().translation.vector;
    }
}
