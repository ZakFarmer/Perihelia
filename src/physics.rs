use std::ops::{Div, Mul};

use bevy::prelude::*;

use crate::body::*;

const CONSTANT_G: f32 = 6.67 * 10e-11;
const DELTA_TIME: f32 = 0.1;

pub fn attract_bodies(
    mut query: Query<(
        &Mass,
        &Radius,
        &GlobalTransform,
        &mut Acceleration,
        &mut LinearMomentum,
    )>,
) {
    let mut iter = query.iter_combinations_mut();
    while let Some(
        [(Mass(m1), Radius(r1), transform1, mut acc1, lin_momentum1), (Mass(m2), Radius(r2), transform2, mut acc2, lin_momentum2)],
    ) = iter.fetch_next()
    {
        let delta = transform2.translation() - transform1.translation();
        let distance_sq: f32 = delta.length_squared();

        let velocity1 = lin_momentum1.0.div(*m1);
        let velocity2 = lin_momentum2.0.div(*m2);

        //println!("Radii: {:?}", (*r1 + *r2));

        if distance_sq.sqrt() <= (*r1 + *r2 + 0.01) {
            let (new_velocity1, new_velocity2) = calculate_velocities_after_collision(
                velocity1,
                velocity2,
                *m1,
                *m2,
                transform1.translation(),
                transform2.translation(),
            );

            acc1.0 = new_velocity1;
            acc2.0 = new_velocity2;

            continue;
        }

        let f = CONSTANT_G / distance_sq;
        let force_unit_mass = delta * f;
        acc1.0 += force_unit_mass * *m2;
        acc2.0 -= force_unit_mass * *m1;
    }
}

pub fn calculate_kinetic_energy(mass: f32, velocity: Vec3) -> f32 {
    0.5 * mass * velocity.dot(velocity)
}

pub fn calculate_momentum(mass: f32, velocity: Vec3) -> Vec3 {
    velocity * mass
}

pub fn calculate_velocities_after_collision(
    v_a: Vec3,
    v_b: Vec3,
    _m_a: f32,
    _m_b: f32,
    x_a: Vec3,
    x_b: Vec3,
) -> (Vec3, Vec3) {
    let mut d = x_a - x_b;

    d = d.normalize();

    let relative_velocity = v_a - v_b;

    let relative_velocity_dot = relative_velocity.dot(d);

    d = d.mul(relative_velocity_dot);

    (-d, d)
}

pub fn calculate_moment_of_inertia_tensor(mass: f32, radius: f32) -> Mat3 {
    let moment_of_inertia = 2. / 5. * mass * radius * radius;

    Mat3 {
        x_axis: Vec3::new(moment_of_inertia, 0., 0.),
        y_axis: Vec3::new(0., moment_of_inertia, 0.),
        z_axis: Vec3::new(0., 0., moment_of_inertia),
    }
}

pub fn calculate_rotational_matrix(orientation: Quat) -> Mat3 {
    let q = orientation;
    let q0 = q.w;
    let q1 = q.x;
    let q2 = q.y;
    let q3 = q.z;

    let r00: f32 = 2. * (q0 * q0 + q1 * q1) - 1.;
    let r01: f32 = 2. * (q1 * q2 - q0 * q3);
    let r02: f32 = 2. * (q1 * q3 + q0 * q2);

    let r10: f32 = 2. * (q1 * q2 + q0 * q3);
    let r11: f32 = 2. * (q0 * q0 + q2 * q2) - 1.;
    let r12: f32 = 2. * (q2 * q3 - q0 * q1);

    let r20: f32 = 2. * (q1 * q3 - q0 * q2);
    let r21: f32 = 2. * (q2 * q3 + q0 * q1);
    let r22: f32 = 2. * (q0 * q0 + q3 * q3) - 1.;

    Mat3 {
        x_axis: Vec3::new(r00, r01, r02),
        y_axis: Vec3::new(r10, r11, r12),
        z_axis: Vec3::new(r20, r21, r22),
    }
}

pub fn integrate(
    mut query: Query<(
        &mut Acceleration,
        &mut Transform,
        &mut Mass,
        &mut Radius,
        &mut Orientation,
        &mut AngularMomentum,
        &mut LinearMomentum,
    )>,
) {
    for (
        mut acceleration,
        mut transform,
        mass,
        _radius,
        _orientation,
        _angular_momentum,
        mut linear_momentum,
    ) in &mut query
    {
        /*let I: Mat3 = calculate_moment_of_inertia_tensor(mass.0, radius.0);
        I.inverse();*/
        linear_momentum.0 = linear_momentum.0 + (acceleration.0 * mass.0);
        let new_pos = transform.translation + (linear_momentum.0 / mass.0) * DELTA_TIME;

        acceleration.0 = Vec3::ZERO;
        transform.translation = new_pos;
    }
}
