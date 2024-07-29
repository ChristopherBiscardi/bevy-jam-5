#![allow(warnings)]
use assets::AssetsPlugin;
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
use blenvy::BlenvyPlugin;
use blenvy_helpers::BlenvyHelpersPlugin;
use collision_layers::CollisionLayersPlugin;
use game_scene::{GameScenePlugin, Player};
use main_menu::MainMenuPlugin;
use navmesh::NavMeshPlugin;
use woodpecker_ui::WoodpeckerUIPlugin;

mod assets;
mod blenvy_helpers;
pub mod collision_layers;
mod game_scene;
mod main_menu;
mod navmesh;
mod widgets;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        let app = app
            .insert_resource(ClearColor(SLATE_200.into()))
            .insert_resource(GameRenderLayer(
                RenderLayers::layer(1),
            ))
            .register_type::<Dof>()
            .insert_resource(Dof {
                focal_distance: 157.5,
                aperture_f_stops: 1.0 / 50.0,
                sensor_height: 0.01866,
                max_circle_of_confusion_diameter: 64.0,
                max_depth: f32::INFINITY,
            })
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

        // #[cfg(feature = "dev")]
        // app.add_plugins(
        //     (
        //         bevy_inspector_egui::quick::WorldInspectorPlugin::new(),
        //     ),
        // );

        app.init_state::<AppState>()
            .add_plugins((
                DefaultPickingPlugins,
                AssetsPlugin,
                AudioPlugin,
                WoodpeckerUIPlugin,
                #[cfg(feature = "with_main_menu")]
                MainMenuPlugin,
                GameScenePlugin,
                CollisionLayersPlugin,
                NavMeshPlugin,
                widgets::CustomWidgetsPlugin,
                BlenvyHelpersPlugin,
                BlenvyPlugin {
                    export_registry: true,
                    ..default()
                },
            ))
            .insert_resource(DebugPickingMode::Normal)
            // .add_systems(
            //     OnEnter(AppState::AssetLoading),
            //     spawn_2d_camera,
            // )
            .add_systems(
                OnEnter(AppState::ErrorScreen),
                on_error,
            )
            .add_systems(
                Update,
                (
                    dof_finder
                        .run_if(in_state(AppState::InGame)),
                    dof_on_change
                        .run_if(resource_changed::<Dof>),
                ),
            )
            .enable_state_scoped_entities::<AppState>();
    }
}

#[derive(Resource, Reflect, Deref)]
#[reflect(Resource)]
struct GameRenderLayer(RenderLayers);

fn dof_finder(
    query: Query<&Transform, With<Camera>>,
    query_player: Query<&Transform, With<Player>>,
    mut dof: ResMut<Dof>,
) {
    let Ok(camera) = query.get_single() else {
        return;
    };
    let Ok(player) = query_player.get_single() else {
        return;
    };

    dof.focal_distance =
        camera.translation.distance(player.translation);
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
struct Dof {
    // pub mode: DepthOfFieldMode,
    pub focal_distance: f32,
    pub sensor_height: f32,
    pub aperture_f_stops: f32,
    pub max_circle_of_confusion_diameter: f32,
    pub max_depth: f32,
}

fn dof_on_change(
    mut query: Query<
        &mut DepthOfFieldSettings,
        With<Camera>,
    >,
    dof: Res<Dof>,
) {
    let Ok(mut settings) = query.get_single_mut() else {
        return;
    };
    // Bokeh only works on Native
    settings.focal_distance = dof.focal_distance;
    settings.aperture_f_stops = dof.aperture_f_stops;
    settings.sensor_height = dof.sensor_height;
    settings.max_circle_of_confusion_diameter =
        dof.max_circle_of_confusion_diameter;
    settings.max_depth = dof.max_depth;
}

#[derive(
    Clone, Eq, PartialEq, Debug, Hash, Default, States,
)]
enum AppState {
    #[default]
    AssetLoading,
    ErrorScreen,
    BevyEngineSplash,
    MainMenu,
    InGame,
}

fn on_error() {
    panic!("here");
    dbg!("error");
}

fn spawn_2d_camera(mut commands: Commands) {
    #[cfg(feature = "with_main_menu")]
    commands.spawn((
        StateScoped(AppState::MainMenu),
        Camera2dBundle {
            camera: Camera {
                // order: 1,
                // if hdr is true on 3d camera, then hdr
                // must be true here too
                hdr: true,
                ..default()
            },
            ..default()
        },
        IsDefaultUiCamera,
    ));
}
