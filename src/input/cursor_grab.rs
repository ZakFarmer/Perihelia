use bevy::prelude::*;

use crate::state::base::SimState;

pub fn cursor_grab_system(
    mut sim_state: ResMut<State<SimState>>,
    mut windows: ResMut<Windows>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let window = windows.get_primary_mut().unwrap();

    if key.just_pressed(KeyCode::Escape) {
        match sim_state.current() {
            SimState::SimRunning => {
                window.set_cursor_lock_mode(false);
                println!("Cursor visibility setting to true...");
                window.set_cursor_visibility(true);

                sim_state.set(SimState::InMenu).unwrap();
            }
            SimState::InMenu => {
                window.set_cursor_lock_mode(true);
                println!("Cursor visibility setting to false...");
                window.set_cursor_visibility(false);

                sim_state.set(SimState::SimRunning).unwrap();
            }
            _ => (),
        }
    }
}
