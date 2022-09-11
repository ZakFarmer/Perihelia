use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Mass(pub f32);

#[derive(Component, Default)]
pub struct Acceleration(pub Vec3);

#[derive(Component, Default)]
pub struct AngularVelocity(pub Vec3);
#[derive(Component, Default)]
pub struct LinearVelocity(pub Vec3);

#[derive(Component)]
pub struct BlackHole;

#[derive(Bundle, Default)]
pub struct BodyBundle {
    #[bundle]
    pub pbr: PbrBundle,
    pub mass: Mass,
    pub angular_velocity: AngularVelocity,
    pub linear_velocity: LinearVelocity,
    pub acceleration: Acceleration,
}
