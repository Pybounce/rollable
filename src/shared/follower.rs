
use bevy::prelude::*;

#[derive(Component)]
pub struct Follower {
    pub target: Entity,
    pub offset: Vec3
}

#[derive(Component)]
pub struct Followable;

pub fn move_followers(
    mut follower_query: Query<(Entity, &mut Transform, &Follower)>,
    followable_query: Query<&Transform, (With<Followable>, Without<Follower>)>,
    mut commands: Commands
) {
    for (entity, mut follower_transform, follower) in &mut follower_query {
        if let Ok(entity_transform) = followable_query.get(follower.target) {
            follower_transform.translation = entity_transform.translation + follower.offset;
        }
        else {
            commands.entity(entity).try_despawn_recursive();
        }
    }
}