use bevy::prelude::*;

use crate::state::base::SimState;

pub fn cursor_grab_system(
    mut sim_state: ResMut<State<SimState>>,
    mut windows: ResMut<Windows>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let window = windows.get_primary_mut().unwrap();

    match sim_state.current() {
        SimState::InMenu => {
            if btn.just_pressed(MouseButton::Left) {
                window.set_cursor_lock_mode(true);
                window.set_cursor_visibility(false);

                sim_state.set(SimState::SimRunning).unwrap();
            }
        }
        _ => {
            if key.just_pressed(KeyCode::Escape) {
                window.set_cursor_lock_mode(false);
                window.set_cursor_visibility(true);
            }
        }
    }

    if btn.just_pressed(MouseButton::Left) {
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
    }

    if key.just_pressed(KeyCode::Escape) {
        window.set_cursor_lock_mode(false);
        window.set_cursor_visibility(true);
    }
}
