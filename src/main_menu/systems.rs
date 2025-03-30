use bevy::prelude::*;

use crate::states::AppState;

use super::components::MainMenuEntity;

pub fn build_main_menu(
    mut commands: Commands
) {
    commands.spawn((
        Text::new("press something idk"),
        MainMenuEntity
    ));
}

pub fn teardown_main_menu(
    mut commands: Commands,
    query: Query<Entity, With<MainMenuEntity>>
) {
    for e in &query {
        commands.entity(e).try_despawn_recursive();
    }
}

pub fn continue_from_main_menu(
    input: Res<ButtonInput<KeyCode>>, 
    mut app_state: ResMut<NextState<AppState>>
) {
    if input.get_just_released().len() > 0 {
        app_state.set(AppState::WorldSelect);
    }
}