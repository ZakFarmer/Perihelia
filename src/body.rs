use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Mass(pub f32);

#[derive(Component, Default)]
pub struct Radius(pub f32);

#[derive(Component, Default)]
pub struct Acceleration(pub Vec3);

#[derive(Component, Default)]
pub struct AngularMomentum(pub Vec3);
#[derive(Component, Default)]
pub struct LinearMomentum(pub Vec3);

#[derive(Component, Default)]
pub struct Orientation(pub Quat);

#[derive(Component)]
pub struct BlackHole;

#[derive(Bundle, Default)]
pub struct BodyBundle {
    #[bundle]
    pub pbr: PbrBundle,
    pub mass: Mass,
    pub radius: Radius,
    pub angular_momentum: AngularMomentum,
    pub linear_momentum: LinearMomentum,
    pub orientation: Orientation,
    pub acceleration: Acceleration,
}
