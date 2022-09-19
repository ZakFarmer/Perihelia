use crate::state::base::SimState;

use bevy::prelude::*;

pub fn esc_to_menu(
    mut keys: ResMut<Input<KeyCode>>,
    mut sim_state: ResMut<State<SimState>>,
    mut windows: ResMut<Windows>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        // Get primary window
        let window = windows.get_primary_mut().unwrap();

        // ESC was pressed, change state to InMenu (or back to SimRunning if current in menu)
        match sim_state.current() {
            SimState::InMenu => {
                // Set the cursor to be locked and invisible (as we are running sim now)
                window.set_cursor_lock_mode(true);
                window.set_cursor_visibility(false);

                sim_state.set(SimState::SimRunning).unwrap();
            }
            _ => {
                // Set the cursor to be unlocked and visible (as we are in menu now)
                window.set_cursor_lock_mode(false);
                window.set_cursor_visibility(true);
                sim_state.set(SimState::InMenu).unwrap();
            }
        }

        // Reset the state of the key
        keys.reset(KeyCode::Escape);
    }
}
