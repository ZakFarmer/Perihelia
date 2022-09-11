use bevy::prelude::*;

use crate::body::*;

const CONSTANT_G: f32 = 6.67 * 10e-11;
const DELTA_TIME: f32 = 0.1;

pub fn attract_bodies(mut query: Query<(&Mass, &GlobalTransform, &mut Acceleration)>) {
    let mut iter = query.iter_combinations_mut();
    while let Some([(Mass(m1), transform1, mut acc1), (Mass(m2), transform2, mut acc2)]) =
        iter.fetch_next()
    {
        let delta = transform2.translation() - transform1.translation();
        let distance_sq: f32 = delta.length_squared();

        let f = CONSTANT_G / distance_sq;
        let force_unit_mass = delta * f;
        acc1.0 += force_unit_mass * *m2;
        acc2.0 -= force_unit_mass * *m1;
    }
}

pub fn integrate(
    mut query: Query<(
        &mut Acceleration,
        &mut Transform,
        &mut AngularVelocity,
        &mut LinearVelocity,
    )>,
) {
    for (mut acceleration, mut transform, mut angular_velocity, mut linear_velocity) in &mut query {
        // verlet integration
        // x(t+dt) = 2x(t) - x(t-dt) + a(t)dt^2 + O(dt^4)
        linear_velocity.0 = linear_velocity.0 + acceleration.0;
        let new_pos = transform.translation + linear_velocity.0 * DELTA_TIME;
        acceleration.0 = Vec3::ZERO;
        transform.translation = new_pos;
    }
}
