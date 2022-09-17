use bevy::prelude::*;

/// A component for a rectangle within the menu
#[derive(Component)]
pub struct MenuRectangle;

/// A component for a label within the menu
#[derive(Component)]
pub struct MenuLabel;

/// A component for a button within the menu
#[derive(Component)]
pub struct MenuButton;

/// A component for Radius text within the menu
#[derive(Component)]
pub struct RadiusText;

/// A component for Mass text within the menu
#[derive(Component)]
pub struct MassText;

/// A component for the FPS debug text
#[derive(Component)]
pub struct FPSText;

/// A component for the delta time debug text
#[derive(Component)]
pub struct DeltaTimeText;
/// A component for the body count debug text
#[derive(Component)]
pub struct CountText;
