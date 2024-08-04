use bevy::{
    diagnostic::{
        DiagnosticsStore, FrameTimeDiagnosticsPlugin,
    },
    prelude::*,
};
use bevy_asset_loader::prelude::*;
use bevy_vello::text::VelloFont;
use iyes_progress::{
    Progress, ProgressCounter, ProgressPlugin,
    ProgressSystem,
};
// use woodpecker_ui::prelude::;

use crate::AppState;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            #[cfg(not(feature = "with_main_menu"))]
            ProgressPlugin::new(AppState::AssetLoading)
                .continue_to(AppState::InGame),
            #[cfg(feature = "with_main_menu")]
            ProgressPlugin::new(AppState::AssetLoading)
                .continue_to(AppState::MainMenu),
            FrameTimeDiagnosticsPlugin,
        ))
        .add_loading_state(
            LoadingState::new(AppState::AssetLoading)
                .load_collection::<TextureAssets>()
                .load_collection::<AudioAssets>()
                .load_collection::<FontAssets>()
                .load_collection::<FontVelloAssets>()
                .load_collection::<PlayerAssets>()
                .load_collection::<NavMeshAssets>()
                .on_failure_continue_to_state(
                    AppState::ErrorScreen,
                ),
        )
        .add_systems(
            Update,
            (
                #[cfg(feature = "long_loading")]
                track_fake_long_task.track_progress(),
                print_progress,
            )
                .chain()
                .run_if(in_state(AppState::AssetLoading))
                .after(LoadingStateSet(
                    AppState::AssetLoading,
                )),
        )
        .add_systems(
            OnExit(AppState::AssetLoading),
            load_new_default,
        );
    }
}

fn load_new_default(
    mut fonts: ResMut<Assets<Font>>,
    new_fonts: Res<FontAssets>,
) {
    // let new_default_font = fonts
    //     .get(&new_fonts.outfit_regular)
    //     .unwrap()
    //     .clone();
    // fonts.insert(&Handle::default(),
    // new_default_font);
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    // #[asset(
    //     path = "audio/UnderwaterAmbience_SFXB.486.ogg"
    // )]
    // pub ambiance:
    //     Handle<bevy_kira_audio::prelude::AudioSource>,
    // #[asset(path = "audio/nr_perc_plop.ogg")]
    // pub plop: Handle<bevy_kira_audio::prelude::AudioSource>,
    #[asset(path = "audio/PE-Data_BW.29124.ogg")]
    pub data_long:
        Handle<bevy_kira_audio::prelude::AudioSource>,
    #[asset(path = "audio/PE-Data_BW.29156.ogg")]
    pub data_short:
        Handle<bevy_kira_audio::prelude::AudioSource>,
    #[asset(path = "audio/ComputerAccessData_HV.163.ogg")]
    pub data_show:
        Handle<bevy_kira_audio::prelude::AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct PlayerAssets {
    #[asset(
        path = "mini_characters_1/character-male-a.glb#Scene0"
    )]
    pub player: Handle<Scene>,
    #[asset(
        path = "mini_characters_1/character-male-a.glb"
    )]
    pub gltf: Handle<Gltf>,
    #[asset(path = "animation_graph/player.animgraph.ron")]
    pub animation_graph: Handle<AnimationGraph>,
}

#[derive(AssetCollection, Resource)]
pub struct NavMeshAssets {
    #[asset(path = "blueprints/navmesh_collection.glb")]
    pub navmesh_gltf: Handle<Gltf>,
    #[asset(path = "materials/Material.001.glb")]
    pub material_001: Handle<Gltf>,
    #[asset(path = "materials/Material.002.glb")]
    pub material_002: Handle<Gltf>,
    #[asset(path = "materials/Material.003.glb")]
    pub material_003: Handle<Gltf>,
    #[asset(path = "materials/water.glb")]
    pub water: Handle<Gltf>,
    #[asset(path = "materials/Pink Fabric.glb")]
    pub pink_fabric: Handle<Gltf>,
    #[asset(path = "materials/Neon Plexi Pink by LP.glb")]
    pub neon_plexi_pink_by_lp: Handle<Gltf>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    // #[asset(path = "images/player.png")]
    // player: Handle<Image>,
    // #[asset(path = "images/tree.png")]
    // tree: Handle<Image>,
    // #[asset(path = "images/female_adventurer_sheet.png")]
    // female_adventurer: Handle<Image>,
    // #[asset(texture_atlas_layout(
    //     tile_size_x = 96,
    //     tile_size_y = 99,
    //     columns = 8,
    //     rows = 1
    // ))]
    // female_adventurer_layout: Handle<TextureAtlasLayout>,
}

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "outfit/Outfit-ExtraBold.ttf")]
    pub outfit_extra_bold: Handle<Font>,
    #[asset(path = "outfit/Outfit-Regular.ttf")]
    pub outfit_regular: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct FontVelloAssets {
    // #[asset(path = "poppins/Poppins-Regular.ttf")]
    // pub outfit_extra_bold: Handle<VelloFont>,
    #[asset(path = "outfit/Outfit-ExtraBold.ttf")]
    pub outfit_extra_bold: Handle<VelloFont>,
    #[asset(path = "outfit/Outfit-Bold.ttf")]
    pub outfit_bold: Handle<VelloFont>,
}

fn print_progress(
    progress: Option<Res<ProgressCounter>>,
    diagnostics: Res<DiagnosticsStore>,
    mut last_done: Local<u32>,
) {
    if let Some(progress) =
        progress.map(|counter| counter.progress())
    {
        if progress.done > *last_done {
            *last_done = progress.done;
            info!(
                "[Frame {}] Changed progress: {:?}",
                diagnostics
                    .get(&FrameTimeDiagnosticsPlugin::FRAME_COUNT)
                    .map(|diagnostic| diagnostic.value().unwrap_or(0.))
                    .unwrap_or(0.),
                progress
            );
        }
    }
}

// Time in seconds to complete a custom
// long-running task. If assets are loaded
// earlier, the current state will not be changed
// until the 'fake long task' is completed (thanks
// to 'iyes_progress')
#[cfg(feature = "long_loading")]
const DURATION_LONG_TASK_IN_SECS: f64 = 4.0;

#[cfg(feature = "long_loading")]
fn track_fake_long_task(time: Res<Time>) -> Progress {
    if time.elapsed_seconds_f64()
        > DURATION_LONG_TASK_IN_SECS
    {
        info!("Long fake task is completed");
        true.into()
    } else {
        false.into()
    }
}
