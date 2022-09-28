use std::ops::Div;

use bevy::prelude::*;

use crate::physics::constants::*;
use crate::physics::helpers::*;
use crate::physics::types::*;
use crate::state::types::SimSettings;
use crate::state::types::SimState;

/// Attracts the bodies towards eachother using Newton's law of universal gravitation
///
/// Also deals with the collision although I'm thinking it might be better to take this out into its own
/// module for more thorough testing.
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

/// Integrates the acceleration to get the velocity and then integrates the velocity to get the position.
/// Currently this doesn't take into account any sort of angular velocity or angular momentum, although
/// this is something I want to implement ASAP.
pub fn integrate(
    sim_settings: ResMut<SimSettings>,
    sim_state: ResMut<State<SimState>>,
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
        match sim_state.current() {
            SimState::SimRunning => (),
            _ => {
                return;
            }
        }

        linear_momentum.0 += acceleration.0 * mass.0;
        let new_pos = transform.translation
            + (linear_momentum.0 / mass.0) * DELTA_TIME * sim_settings.timescale;

        acceleration.0 = Vec3::ZERO;
        transform.translation = new_pos;
    }
}
