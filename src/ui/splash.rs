use bevy::prelude::*;

use crate::state::types::SimState;

use super::despawn_screen::despawn_screen;

// This plugin will display a splash screen with Bevy logo for 1 second before switching to the menu
pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        // As this plugin is managing the splash screen, it will focus on the state `GameState::Splash`
        app
            // When entering the state, spawn everything needed for this screen
            .add_system_set(SystemSet::on_enter(SimState::SplashScreen).with_system(splash_setup))
            // While in this state, run the `countdown` system
            .add_system_set(SystemSet::on_update(SimState::SplashScreen).with_system(countdown))
            // When exiting the state, despawn everything that was spawned for this screen
            .add_system_set(
                SystemSet::on_exit(SimState::SplashScreen)
                    .with_system(despawn_screen::<OnSplashScreen>),
            );
    }
}

// Tag component used to tag entities added on the splash screen
#[derive(Component)]
struct OnSplashScreen;

// Newtype to use a `Timer` for this screen as a resource
#[derive(Component, Deref, DerefMut)]
pub struct SplashTimer(Timer);

pub fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("images/splash.png");

    commands
        .spawn()
        .insert_bundle(ImageBundle {
            style: Style {
                // This will center the logo
                margin: UiRect::all(Val::Auto),
                // This will set the logo to be 200px wide, and auto adjust its height
                size: Size::new(Val::Px(200.0), Val::Auto),
                ..default()
            },
            image: UiImage(icon),
            ..default()
        })
        .insert(OnSplashScreen);

    commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, false)));
}

// Tick the timer, and change state when finished
pub fn countdown(
    mut sim_state: ResMut<State<SimState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        sim_state.set(SimState::SimRunning).unwrap();
    }
}
