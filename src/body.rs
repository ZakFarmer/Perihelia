use bevy::prelude::*;
use serde::Serialize;

#[derive(Component, Default, Reflect)]
pub struct Mass(pub f32);

#[derive(Component, Default, Reflect)]
pub struct Radius(pub f32);

#[derive(Component, Default, Reflect)]
pub struct Acceleration(pub Vec3);

#[derive(Component, Default, Reflect)]
pub struct AngularMomentum(pub Vec3);
#[derive(Component, Default, Reflect)]
pub struct LinearMomentum(pub Vec3);

#[derive(Component, Default, Reflect)]
pub struct Orientation(pub Quat);

#[derive(Component)]
pub struct BlackHole;

#[derive(Component, Reflect)]
pub struct Body {
    pub bundle: BodyBundle,
}

#[derive(Bundle, Default, Reflect)]
pub struct BodyBundle {
    #[bundle]
    #[reflect(ignore)]
    pub pbr: PbrBundle,
    pub mass: Mass,
    pub radius: Radius,
    pub angular_momentum: AngularMomentum,
    pub linear_momentum: LinearMomentum,
    pub orientation: Orientation,
    pub acceleration: Acceleration,
}
