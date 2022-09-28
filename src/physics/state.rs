#[derive(Resource, Default)]
pub struct SimulationState {
    timescale: f32,
    particle_count: usize,
}
