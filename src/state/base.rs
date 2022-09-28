use bevy::prelude::*;

use super::types::{SimSettings, SimState};

pub fn setup_state(
    mut sim_state: ResMut<State<SimState>>,
    mut windows: ResMut<Windows>,
    mut sim_settings: ResMut<SimSettings>,
) {
    let window = windows.get_primary_mut().unwrap();

    window.set_cursor_lock_mode(true);
    window.set_cursor_visibility(false);
}
