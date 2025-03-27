
use bevy::prelude::*;

use super::components::SharedAssets;

pub fn load_stage_assets(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let base_tex: Handle<Image> = server.load("colour_palette_01.png");
    let base_material = materials.add(StandardMaterial {
        perceptual_roughness: 1.0,
        base_color_texture: base_tex.into(),
        ..default()
    });

    commands.insert_resource::<SharedAssets>(SharedAssets {
        base_material: base_material,
    });
}