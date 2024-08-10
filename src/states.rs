use bevy::prelude::*;

pub struct WashCycleStatesPlugin;

impl Plugin for WashCycleStatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_sub_state::<IsPaused>()
            .add_sub_state::<GameMode>()
            .enable_state_scoped_entities::<AppState>()
            .enable_state_scoped_entities::<IsPaused>()
            .enable_state_scoped_entities::<GameMode>();
    }
}

#[derive(
    Clone, Eq, PartialEq, Debug, Hash, Default, States,
)]
pub enum AppState {
    #[default]
    AssetLoading,
    ErrorScreen,
    BevyEngineSplash,
    MainMenu,
    InGame,
}

#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    Eq,
    PartialEq,
    Hash,
    SubStates,
)]
#[source(AppState = AppState::InGame)]
pub enum IsPaused {
    #[default]
    Running,
    Paused,
}

#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    Eq,
    PartialEq,
    Hash,
    SubStates,
)]
#[source(AppState = AppState::InGame)]
pub enum GameMode {
    #[default]
    Regular,
    VirtualGridPlacement,
}
