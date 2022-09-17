use std::ops::Mul;

use bevy::prelude::*;

/// Kronecker delta: 1 if i == j, 0 otherwise
pub fn kronecker_delta(i: u8, j: u8) -> u8 {
    (i == j) as u8
}

/// Calculates the kinetic energy given a mass and vector of Velocity
pub fn calculate_kinetic_energy(mass: f32, velocity: Vec3) -> f32 {
    0.5 * mass * velocity.dot(velocity)
}

/// Calculates the momentum given a mass and vector of Velocity
pub fn calculate_momentum(mass: f32, velocity: Vec3) -> Vec3 {
    velocity * mass
}

/// Calculates the velocities after a collision given the velocities of the two bodies,
/// their masses, and their positions.
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

/// Calculates the tensor representing the moment of intertia of a body given its mass and radius (assumed to be a sphere)
pub fn calculate_moment_of_inertia_tensor(mass: f32, radius: f32) -> Mat3 {
    let moment_of_inertia = 2. / 5. * mass * radius * radius;

    Mat3 {
        x_axis: Vec3::new(moment_of_inertia, 0., 0.),
        y_axis: Vec3::new(0., moment_of_inertia, 0.),
        z_axis: Vec3::new(0., 0., moment_of_inertia),
    }
}

/// Converts the orientation quaternion into a rotation matrix
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
