use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum SimState {
    InMenu,
    SimRunning,
    SimPaused,
    SimLoading,
}

pub fn setup_state(mut sim_state: ResMut<State<SimState>>, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();

    window.set_cursor_lock_mode(true);
    window.set_cursor_visibility(false);
}
