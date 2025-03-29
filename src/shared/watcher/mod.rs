
use bevy::prelude::*;

/// Placing on a camera, should make it look at the given target entity
#[derive(Component)]
pub struct Watcher {
    pub target: Entity,
    pub speed: f32
}

#[derive(Component)]
pub struct Watchable;

pub fn watch_target(
    mut watcher_query: Query<(&mut Transform, &Watcher)>,
    watchable_query: Query<&Transform, (With<Watchable>, Without<Watcher>)>,
    time: Res<Time>
) {
    for (mut watcher_transform, watcher) in &mut watcher_query {
        if let Ok(watchable_transform) = watchable_query.get(watcher.target) {
            let dir = (watchable_transform.translation - watcher_transform.translation).normalize_or_zero();
            let watcher_dir = watcher_transform.forward().normalize_or_zero();
            let target_rot = Quat::from_rotation_arc(watcher_dir, dir);

            
            let angle = watcher_transform.rotation.angle_between(target_rot * watcher_transform.rotation);

            let step = (watcher.speed * time.delta_secs()).min(angle);
            watcher_transform.rotation = watcher_transform.rotation.slerp(target_rot * watcher_transform.rotation, step);
        }
    }
}