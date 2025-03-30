
use avian3d::prelude::CollidingEntities;
use bevy::prelude::*;

use crate::{player::components::Player, stage::components::LoadStageConfig, states::AppState};

#[derive(Component)]
pub struct StageTeleport {
    pub stage_id: usize
}


pub fn teleport_player_to_stage(
    player_query: Query<&CollidingEntities, With<Player>>,
    teleporter_query: Query<&StageTeleport, Without<Player>>,
    mut app_state: ResMut<NextState<AppState>>,
    mut commands: Commands
) {
    for colliding_entities in &player_query {
        for colliding_entity in colliding_entities.iter() {
            if let Ok(tele) = teleporter_query.get(*colliding_entity) {
                commands.insert_resource(LoadStageConfig::new(tele.stage_id));
                app_state.set(AppState::InStage);
                return;
            }
        }

    }
}