use crate::state::base::SimState;

use bevy::prelude::*;

pub fn esc_to_menu(mut keys: ResMut<Input<KeyCode>>, mut sim_state: ResMut<State<SimState>>) {
    if keys.just_pressed(KeyCode::Escape) {
        // ESC was pressed, change state to InMenu (or back to SimRunning if current in menu)
        match sim_state.current() {
            SimState::InMenu => {
                sim_state.set(SimState::SimRunning).unwrap();
            }
            _ => {
                sim_state.set(SimState::InMenu).unwrap();
            }
        }

        // Reset the state of the key
        keys.reset(KeyCode::Escape);
    }
}
