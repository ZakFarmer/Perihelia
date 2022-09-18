#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum SimState {
    InMenu,
    SimRunning,
    SimPaused,
    SimLoading,
}
