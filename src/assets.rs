use bevy::diagnostic::{
    DiagnosticsStore, FrameTimeDiagnosticsPlugin,
};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use iyes_progress::{
    Progress, ProgressCounter, ProgressPlugin,
    ProgressSystem,
};
use woodpecker_ui::prelude::VelloFont;

use crate::AppState;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ProgressPlugin::new(AppState::AssetLoading)
                .continue_to(AppState::MainMenu),
            FrameTimeDiagnosticsPlugin,
        ))
        .add_loading_state(
            LoadingState::new(AppState::AssetLoading)
                .load_collection::<TextureAssets>()
                .load_collection::<AudioAssets>()
                .load_collection::<FontAssets>()
                .load_collection::<FontVelloAssets>(),
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
        );
    }
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(
        path = "audio/UnderwaterAmbience_SFXB.486.ogg"
    )]
    pub ambiance: Handle<AudioSource>,
    #[asset(path = "audio/nr_perc_plop.ogg")]
    pub plop: Handle<AudioSource>,
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
}

#[derive(AssetCollection, Resource)]
pub struct FontVelloAssets {
    #[asset(path = "outfit/Outfit-ExtraBold.ttf")]
    pub outfit_extra_bold: Handle<VelloFont>,
    #[asset(path = "poppins/Poppins-Regular.ttf")]
    pub poppins_regular: Handle<VelloFont>,
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

// Time in seconds to complete a custom long-running task.
// If assets are loaded earlier, the current state will not
// be changed until the 'fake long task' is completed (thanks to 'iyes_progress')
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
