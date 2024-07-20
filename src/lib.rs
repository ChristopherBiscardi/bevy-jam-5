use assets::AssetsPlugin;
use bevy::{
    color::palettes::tailwind::SLATE_950, prelude::*,
};
use bevy_mod_picking::{
    debug::DebugPickingMode, DefaultPickingPlugins,
};
use main_menu::MainMenuPlugin;
use woodpecker_ui::WoodpeckerUIPlugin;

mod assets;
mod main_menu;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        let app = app
            .insert_resource(ClearColor(SLATE_950.into()))
            .add_plugins(DefaultPlugins);

        #[cfg(feature = "dev")]
        app
            .add_plugins((
                bevy_inspector_egui::quick::WorldInspectorPlugin::new(),
                DefaultPickingPlugins
            ))
            .insert_resource(DebugPickingMode::Normal);

        app.init_state::<AppState>()
            .add_plugins((
                AssetsPlugin,
                MainMenuPlugin,
                WoodpeckerUIPlugin,
            ))
            .add_systems(
                OnEnter(AppState::AssetLoading),
                spawn_camera,
            );
    }
}

#[derive(
    Clone, Eq, PartialEq, Debug, Hash, Default, States,
)]
enum AppState {
    #[default]
    AssetLoading,
    KennyJamSplash,
    BevyEngineSplash,
    MainMenu,
    InGame,
}

fn spawn_camera(mut commands: Commands) {
    // commands.spawn(Camera3dBundle::default());
    commands.spawn(Camera2dBundle::default());
}
