#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum SimState {
    InMenu,
    SimRunning,
    SimPaused,
    Loading,
    SplashScreen,
}

pub struct SimSettings {
    pub timescale: f32,
}
