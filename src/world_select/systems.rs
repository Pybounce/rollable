
use std::f32::consts::FRAC_PI_4;

use avian3d::{math::{FRAC_PI_2, PI}, prelude::{AngularVelocity, NoAutoCenterOfMass, RigidBody}};
use bevy::{math::{primitives, VectorSpace}, prelude::*};

use crate::{loading::components::SharedAssets, stage::stage_builder::{build_air_loon, build_floor, build_goal, build_tree_m, Floor}, states::AppState};

use super::{components::{WorldType, *}, functions::*};

pub fn init_world_select(
    mut commands: Commands,
    server: Res<AssetServer>,
    shared_assets: Res<SharedAssets>,
    mut cam_query: Query<(&mut Transform, Entity), With<Camera3d>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>
) {

    let mut controller = WorldSelectController::default();
    controller.add_world(WorldType::Grasslands, build_world_one(&mut commands, &server, &shared_assets));
    controller.add_world(WorldType::SomethingElse, build_world_two(&mut commands, &server, &shared_assets));

    let water_mat = StandardMaterial {
        perceptual_roughness: 0.6,
        base_color: Color::linear_rgb(35.0/255.0, 137.0/255.0, 218.0/255.0),
        ..default()
    };

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10000.0, 10000.0))),
        MeshMaterial3d(mats.add(water_mat)),
        Transform::from_translation(Vec3::new(0.0, -2.0, 0.0)),
        WorldSelectEntity
    ));


    if let Ok((mut cam_transform, cam_entity)) = cam_query.get_single_mut() {
        cam_transform.translation = Vec3::new(0.0, 20.0, 0.0);
        cam_transform.look_at(island_pos_from_world(WorldType::Grasslands), Vec3::Y);
        //commands.entity(cam_entity).try_insert(Watcher {
        //    target: controller.current_world_entity(),
        //    speed: 10.0
        //});
    }

    commands.spawn((
        Text::new("Grasslands"),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(20.0),
            left: Val::Percent(50.0),
            ..default()
        },
        WorldSelectEntity
    ));

    commands.insert_resource(controller);

}

pub fn teardown_world_select(
    mut commands: Commands,
    query: Query<Entity, With<WorldSelectEntity>>
) {
    for e in &query {
        commands.entity(e).try_despawn_recursive();
    }
    commands.remove_resource::<WorldSelectController>();
}

pub fn cycle_selected_world(
    mut controller: ResMut<WorldSelectController>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::ArrowRight) {
        controller.cycle_next_world();
    }
    if input.just_pressed(KeyCode::ArrowLeft) {
        controller.cycle_next_world();
    }
}

pub fn select_world(
    mut app_state: ResMut<NextState<AppState>>
) {
    
}

pub fn move_world_select_cam(
    mut cam_query: Query<&mut Transform, With<Camera>>,
    controller: Res<WorldSelectController>,
    time: Res<Time>
) {
    let mut cam_transform = cam_query.single_mut();

    let island_flattened_dir = (island_pos_from_world(controller.current_world).normalize()).xz().normalize().extend(0.0).xzy();
    let cam_flattened_dir = (cam_transform.forward().as_vec3()).xz().normalize().extend(0.0).xzy();
        
    let angle = cam_flattened_dir.angle_between(island_flattened_dir);
    let cross = cam_flattened_dir.cross(island_flattened_dir).y;
    
    let real = (controller.min_camera_speed.max(controller.max_camera_speed * angle) * time.delta_secs()).min(angle);
    
    cam_transform.rotate_y(real * cross.signum());
}



