
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_hanabi::prelude::*;

use crate::{loading::components::SharedAssets, physics::GamePhysicsLayer, shared::{bouncy::components::Bounceable, follower::{Followable, Follower}}};

use super::{components::{JumpController, Player, PlayerController}, particles::components::*};

#[derive(Component)]
#[require(Transform)]
pub struct PlayerSpawner;

pub fn try_spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    particles: Res<PlayerParticleEffects>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    player_query: Query<(), With<Player>>,
    spawner_query: Query<&Transform, (With<PlayerSpawner>, Without<Player>)>
) {
    if player_query.iter().count() == 0 {
        if let Ok(spawner_t) = spawner_query.get_single() {
            let mesh = meshes.add(Sphere::default());
            let material = materials.add(StandardMaterial::default());

            let player_entity = commands.spawn((
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
                SpeculativeMargin(2.0),
                Followable
            )).id();
            commands.spawn((
                ParticleEffect::new(particles.player_running_eff_handle.clone()),
                Follower {
                    target: player_entity,
                    offset: Vec3::new(0.0, -0.5, 0.0),
                },
                PlayerGroundRunningParticleEmiter
            ));
            commands.spawn((
                ParticleEffect::new(particles.player_landing_eff_handle.clone()),
                Follower {
                    target: player_entity,
                    offset: Vec3::new(0.0, -0.5, 0.0),
                },
                PlayerGroundLandingParticleEmiter
            ));
            
        }
    }
}