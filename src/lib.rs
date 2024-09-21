#![feature(extract_if)]
#![allow(warnings)]
use animation_graph_processing::AnimationGraphProcessingPlugin;
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
use custom_materials::CustomMaterialsPlugin;
use customer_npc::CustomerNpcPlugin;
use inventory::InventoryPlugin;
use persistent_id::PersistentIdPlugin;
use woodpecker_ui::{RenderSettings, WoodpeckerUIPlugin};

use crate::{
    assets::WashCycleAssetsPlugin,
    blenvy_helpers::BlenvyHelpersPlugin,
    camera::WashCycleCameraPlugin,
    collision_layers::CollisionLayersPlugin,
    controls::ControlsPlugin,
    game_scene::{GameScenePlugin, Player},
    grid::GridPlugin,
    main_menu::MainMenuPlugin,
    navmesh::NavMeshPlugin,
    states::{AppState, WashCycleStatesPlugin},
};

mod animation_graph_processing;
mod assets;
mod blenvy_helpers;
mod camera;
pub mod collision_layers;
mod controls;
mod custom_materials;
mod customer_npc;
mod game_scene;
mod grid;
mod inventory;
mod main_menu;
mod navmesh;
mod persistent_id;
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
                // only export registry if the debug
                // assertions are on
                // giving us effectively only
                export_registry: cfg!(
                    feature = "only_write_registry"
                ) || cfg!(
                    debug_assertions
                ),
                ..default()
            },
            CustomerNpcPlugin,
            GridPlugin,
            ControlsPlugin,
            (
                WashCycleAssetsPlugin,
                WashCycleCameraPlugin,
                widgets::WashCycleWidgetsPlugin,
                AnimationGraphProcessingPlugin,
                PersistentIdPlugin,
                InventoryPlugin,
                CustomMaterialsPlugin,
            ),
        ))
        // .insert_resource(DebugPickingMode::Normal)
        .add_systems(
            OnEnter(AppState::ErrorScreen),
            on_error,
        );

        #[cfg(feature = "only_write_registry")]
        app.add_systems(
            PostStartup,
            write_registry_then_exit,
        );
    }
}

fn write_registry_then_exit(
    mut app_exit: EventWriter<AppExit>,
) {
    app_exit.send(AppExit::Success);
}

fn on_error() {
    panic!("here");
    dbg!("error");
}
