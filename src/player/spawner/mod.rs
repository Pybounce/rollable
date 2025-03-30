
use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{physics::GamePhysicsLayer, shared::bouncy::components::Bounceable};

use super::components::{JumpController, Player, PlayerController};

#[derive(Component)]
#[require(Transform)]
pub struct PlayerSpawner;

pub fn try_spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    player_query: Query<(), With<Player>>,
    spawner_query: Query<&Transform, (With<PlayerSpawner>, Without<Player>)>
) {
    if player_query.iter().count() == 0 {
        if let Ok(spawner_t) = spawner_query.get_single() {
            let mesh = meshes.add(Sphere::default());
            let material = materials.add(StandardMaterial::default());

            commands.spawn((
                Mesh3d(mesh),
                MeshMaterial3d(material),
                Player,
                Transform::from_translation(spawner_t.translation),
                Collider::sphere(0.5),
                RigidBody::Dynamic,
                GravityScale(6.0),
                LinearVelocity::default(),
                PlayerController::default(),
                JumpController::default(),
                CollisionLayers::new(GamePhysicsLayer::Ball, [GamePhysicsLayer::Ground]),
                CollidingEntities::default(),
                Bounceable,
                SpeculativeMargin(2.0)
            ));
        }
    }
}