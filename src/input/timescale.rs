use bevy::prelude::*;

use crate::{
    input::constants::TIMESCALE_STEP,
    state::types::{SimSettings, SimState},
};

pub fn change_timescale_system(
    mut sim_settings: ResMut<SimSettings>,
    mut sim_state: ResMut<State<SimState>>,
    key: Res<Input<KeyCode>>,
) {
    if key.just_pressed(KeyCode::Period) {
        println!("{:?}", sim_settings.timescale);

        if sim_settings.timescale + TIMESCALE_STEP >= 1000.0 {
            sim_settings.timescale = 1.0;
        }

        sim_settings.timescale += TIMESCALE_STEP;
    }

    if key.just_pressed(KeyCode::Comma) {
        println!("{:?}", sim_settings.timescale);

        if sim_settings.timescale - TIMESCALE_STEP <= 0.0 {
            sim_settings.timescale = 1.0;
        }

        sim_settings.timescale -= TIMESCALE_STEP;
    }
}
