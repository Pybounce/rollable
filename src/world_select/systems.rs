
use avian3d::{math::FRAC_PI_2, prelude::{AngularVelocity, NoAutoCenterOfMass, RigidBody}};
use bevy::{math::VectorSpace, prelude::*};

use crate::{loading::components::SharedAssets, stage::stage_builder::{build_floor, build_goal, build_tree_m, Floor}, states::AppState};

use super::components::{World, *};

pub fn build_world_select(
    mut commands: Commands,
    server: Res<AssetServer>,
    shared_assets: Res<SharedAssets>,
    mut cam_query: Query<&mut Transform, With<Camera3d>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>
) {

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


    if let Ok(mut cam_transform) = cam_query.get_single_mut() {
        cam_transform.translation = Vec3::new(12.0, 20.0, 35.0);
        cam_transform.look_at(Vec3::ZERO, Vec3::Y);
    }
    build_world_one(&mut commands, &server, &shared_assets);

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
}

pub fn teardown_world_select(
    mut commands: Commands,
    query: Query<Entity, With<WorldSelectEntity>>
) {
    for e in &query {
        commands.entity(e).try_despawn_recursive();
    }
}

pub fn select_world(
    mut app_state: ResMut<NextState<AppState>>
) {
    
}

fn build_world(world: World) {
    match world {
        World::Grasslands => build_world_one,
        World::SomethingElse => todo!(),
    }
}

fn build_world_one<'c>(
    commands: &'c mut Commands, 
    server: & Res<AssetServer>, 
    shared_assets: & SharedAssets,
) {
    let floor0 = build_floor(commands, server, shared_assets, Vec3::ZERO, Vec3::new(30.0, 2.0, 30.0), Floor::Octagon).id();

    let tree = build_tree_m(commands, server, shared_assets, Vec3::new(3.0, 0.0, 0.0)).id();
    let goal = build_goal(commands, server, shared_assets, Vec3::new(-3.0, 0.0, 0.0)).id();

    let mut entity = commands.spawn((
        WorldSelectEntity,
        Transform::default(),
        RigidBody::Kinematic,
        NoAutoCenterOfMass,
        //AngularVelocity(Vec3::new(0.0, 0.5, 0.0))
    ));
    entity.add_child(floor0);
    entity.add_child(tree);
    entity.add_child(goal);

    
}





