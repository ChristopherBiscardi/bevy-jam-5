#![allow(warnings)]
use assets::WashCycleAssetsPlugin;
use avian3d::{
    prelude::RigidBody,
    sync::ancestor_marker::AncestorMarker,
};
use bevy::{
    asset::AssetMetaCheck,
    color::palettes::tailwind::*,
    core_pipeline::{
        bloom::BloomSettings,
        dof::{DepthOfFieldMode, DepthOfFieldSettings},
        tonemapping::Tonemapping,
    },
    pbr::VolumetricFogSettings,
    prelude::*,
    render::view::RenderLayers,
};
use bevy_kira_audio::prelude::*;
use bevy_mod_picking::{
    debug::DebugPickingMode, DefaultPickingPlugins,
};
use bevy_picking_avian::AvianBackendSettings;
use bevy_vello::render::VelloRenderSettings;
use blenvy::BlenvyPlugin;
use camera::WashCycleCameraPlugin;
use states::WashCycleStatesPlugin;
use woodpecker_ui::{RenderSettings, WoodpeckerUIPlugin};

use crate::{
    blenvy_helpers::BlenvyHelpersPlugin,
    collision_layers::CollisionLayersPlugin,
    game_scene::{GameScenePlugin, Player},
    grid::GridPlugin,
    leafwing_test::LeafwingTestPlugin,
    main_menu::MainMenuPlugin,
    navmesh::NavMeshPlugin,
    states::AppState,
};

mod assets;
mod blenvy_helpers;
mod camera;
pub mod collision_layers;
mod game_scene;
mod grid;
mod leafwing_test;
mod main_menu;
mod navmesh;
mod states;
mod widgets;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        let app = app
            .insert_resource(ClearColor(SLATE_200.into()))
            .add_plugins(
                DefaultPlugins.set(WindowPlugin {
                    primary_window: Window {
                        title: "Wash Cycle".to_string(),
                        ..default()
                    }
                    .into(),
                    ..default()
                }),
            );

        #[cfg(feature = "dev")]
        app.add_plugins(
            (
                bevy_inspector_egui::quick::WorldInspectorPlugin::new(),
            ),
        );

        app.add_plugins((
            WashCycleStatesPlugin,
            DefaultPickingPlugins,
            AudioPlugin,
            WoodpeckerUIPlugin {
                render_settings: RenderSettings {
                    layer: RenderLayers::layer(1),
                    ..default()
                },
            },
            #[cfg(feature = "with_main_menu")]
            MainMenuPlugin,
            GameScenePlugin,
            CollisionLayersPlugin,
            NavMeshPlugin,
            BlenvyHelpersPlugin,
            BlenvyPlugin {
                // TODO: when releasing this should be
                // turned off
                export_registry: true,
                ..default()
            },
            GridPlugin,
            LeafwingTestPlugin,
            (
                WashCycleAssetsPlugin,
                WashCycleCameraPlugin,
                widgets::WashCycleWidgetsPlugin,
            ),
        ))
        .insert_resource(DebugPickingMode::Normal)
        .add_systems(
            OnEnter(AppState::ErrorScreen),
            on_error,
        );
    }
}

fn on_error() {
    panic!("here");
    dbg!("error");
}
