
use bevy::prelude::*;

use super::components::OffsetMover;

pub fn move_offset_movers(
    mut query: Query<(&mut OffsetMover, &mut Transform)>,
    time: Res<Time>
) {
    for (mut offset_mover, mut transform) in &mut query {
        transform.translation -= offset_mover.current_offset();
        offset_mover.progress(time.delta_secs());
        transform.translation += offset_mover.current_offset();
    }
}