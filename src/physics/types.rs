use bevy::prelude::*;

/// A component for the body's mass (kg)
#[derive(Component, Default)]
pub struct Mass(pub f32);

/// A component for the body's radius (m)
#[derive(Component, Default)]
pub struct Radius(pub f32);

/// A component for the body's acceleration (m/s^2)
#[derive(Component, Default)]
pub struct Acceleration(pub Vec3);

/// A component for the body's angular momentum (kg m^2/s)
#[derive(Component, Default)]
pub struct AngularMomentum(pub Vec3);

/// A component for the body's linear momentum (kg m/s)
#[derive(Component, Default)]
pub struct LinearMomentum(pub Vec3);

/// A component for the body's orientation (quaternion)
#[derive(Component, Default)]
pub struct Orientation(pub Quat);

/// A bundle for the body's physics components
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
