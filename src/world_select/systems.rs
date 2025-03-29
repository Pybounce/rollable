
use avian3d::{math::{FRAC_PI_2, PI}, prelude::{AngularVelocity, NoAutoCenterOfMass, RigidBody}};
use bevy::{math::{primitives, VectorSpace}, prelude::*};

use crate::{loading::components::SharedAssets, shared::watcher::{Watchable, Watcher}, stage::stage_builder::{build_floor, build_goal, build_tree_m, Floor}, states::AppState};

use super::components::{WorldId, *};

pub fn init_world_select(
    mut commands: Commands,
    server: Res<AssetServer>,
    shared_assets: Res<SharedAssets>,
    mut cam_query: Query<(&mut Transform, Entity), With<Camera3d>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>
) {

    let mut controller = WorldSelectController::default();
    controller.add_world(WorldId::Grasslands, build_world_one(&mut commands, &server, &shared_assets));
    controller.add_world(WorldId::SomethingElse, build_world_two(&mut commands, &server, &shared_assets));

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
        commands.entity(cam_entity).try_insert(Watcher {
            target: controller.current_world_entity(),
            speed: 10.0
        });
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

pub fn select_world(
    mut app_state: ResMut<NextState<AppState>>
) {
    
}

fn build_world_one<'c>(
    commands: &'c mut Commands, 
    server: & Res<AssetServer>, 
    shared_assets: & SharedAssets,
) -> Entity {
    let floor0 = build_floor(commands, server, shared_assets, Vec3::ZERO, Vec3::new(30.0, 2.0, 30.0), Floor::Octagon).id();

    let tree = build_tree_m(commands, server, shared_assets, Vec3::new(3.0, 0.0, 0.0)).id();
    let goal = build_goal(commands, server, shared_assets, Vec3::new(-3.0, 0.0, 0.0)).id();

    let pos = Quat::from_rotation_y(0.0) * Vec3::new(0.0, 0.0, -35.0);
    let mut entity = commands.spawn((
        WorldSelectEntity,
        Transform::from_translation(pos),
        RigidBody::Kinematic,
        NoAutoCenterOfMass,
        Watchable
        //AngularVelocity(Vec3::new(0.0, 0.5, 0.0))
    ));
    entity.add_child(floor0);
    entity.add_child(tree);
    entity.add_child(goal);

    return entity.id();
}

fn build_world_two<'c>(
    commands: &'c mut Commands, 
    server: & Res<AssetServer>, 
    shared_assets: & SharedAssets,
) -> Entity {
    let floor0 = build_floor(commands, server, shared_assets, Vec3::ZERO, Vec3::new(30.0, 2.0, 30.0), Floor::Octagon).id();

    let tree = build_tree_m(commands, server, shared_assets, Vec3::new(3.0, 0.0, 0.0)).id();
    let goal = build_goal(commands, server, shared_assets, Vec3::new(-3.0, 0.0, 0.0)).id();

    let pos = Quat::from_rotation_y(90.0 * (PI / 180.0)) * Vec3::new(0.0, 0.0, -35.0);
    let mut entity = commands.spawn((
        WorldSelectEntity,
        Transform::from_translation(pos),
        RigidBody::Kinematic,
        NoAutoCenterOfMass,
        Watchable,
        //AngularVelocity(Vec3::new(0.0, 0.5, 0.0))
    ));
    entity.add_child(floor0);
    entity.add_child(tree);
    entity.add_child(goal);

    return entity.id();
}




