use bevy::prelude::*;

pub struct MenuMaterials {
    pub root: Handle<ColorMaterial>,
    pub border: Handle<ColorMaterial>,
    pub menu: Handle<ColorMaterial>,
    pub button: Handle<ColorMaterial>,
    pub button_hovered: Handle<ColorMaterial>,
    pub button_pressed: Handle<ColorMaterial>,
    pub button_text: Color,
}

impl FromWorld for MenuMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        MenuMaterials {
            root: materials.add(Color::NONE.into()),
            border: materials.add(Color::rgb(0.65, 0.65, 0.65).into()),
            menu: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            button: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            button_hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            button_pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
            button_text: Color::WHITE,
        }
    }
}
