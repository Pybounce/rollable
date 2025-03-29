
use bevy::prelude::*;

use crate::shared::death::MarkOfDeath;

use super::components::Player;

pub fn kill_player(
    mut commands: Commands,
    query: Query<Entity, (With<MarkOfDeath>, With<Player>)>
) {
    for e in &query {
        commands.entity(e).try_despawn_recursive();
    }
}