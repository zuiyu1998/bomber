use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub mod systems;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum GameState {
    None,
    Loading,
    InGame,
}

#[derive(Debug, Default)]
pub struct LoadAssets(pub Vec<HandleUntyped>);

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<LoadAssets>();

        app.add_loopless_state(GameState::Loading);

        app.add_system(systems::wait_resource_load);

        app.add_enter_system(GameState::Loading, systems::load_game_resource);

        app.add_enter_system(GameState::InGame, systems::spawn_map);
    }
}
