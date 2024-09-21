use avian3d::prelude::*;
use bevy::{
    color::palettes::{css::ORANGE_RED, tailwind::*},
    core_pipeline::{
        bloom::BloomSettings,
        dof::{DepthOfFieldMode, DepthOfFieldSettings},
        tonemapping::Tonemapping,
    },
    math::vec3,
    pbr::{
        CascadeShadowConfigBuilder, NotShadowCaster,
        VolumetricFogSettings, VolumetricLight,
    },
    prelude::*,
    render::primitives::Aabb,
    scene::SceneInstance,
};
use bevy_mod_raycast::prelude::RaycastSource;
use bevy_tnua::{prelude::*, TnuaAnimatingState};
use bevy_tnua_avian3d::*;
use blenvy::{
    BlueprintInfo, GameWorldTag, HideUntilReady,
    SpawnBlueprint,
};
use leafwing_input_manager::InputManagerBundle;
use rand::Rng;
use std::{
    f32::consts::{FRAC_PI_2, FRAC_PI_8, PI},
    time::Duration,
};

use game_menu::spawn_game_menu;
use tnua_animation::{AnimationState, TnuaAnimationPlugin};

pub mod game_menu;
mod tnua_animation;

use crate::{
    animation_graph_processing::{
        AnimationsList, LoadedAnimationGraphs,
    },
    assets::PlayerAssets,
    collision_layers::{CollisionGrouping, GameLayer},
    controls::PlayerAction,
    customer_npc::{
        CustomerNpc, CustomerNpcAnimationNames,
    },
    inventory::Inventory,
    navmesh::{Obstacle, Spawner},
    states::{AppState, IsPaused},
};

pub const PLAYER_COLLIDER_HEIGHT: f32 = 0.4;
pub const PLAYER_FLOATING_HEIGHT: f32 =
    PLAYER_COLLIDER_HEIGHT + 0.05;

pub struct GameScenePlugin;

impl Plugin for GameScenePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AmbientLight {
            color: ORANGE_RED.into(),
            brightness: 420.0,
        })
        .init_resource::<DropTimer>()
        .register_type::<ExampleAnimationWeights>()
        .register_type::<GameOverSensor>()
        .register_type::<WashingMachine>()
        .add_plugins((
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
            TnuaControllerPlugin::default(),
            TnuaAvian3dPlugin::default(),
            TnuaAnimationPlugin,
        ))
        .add_systems(
            OnEnter(AppState::InGame),
            (
                spawn_player,
                setup_level,
                spawn_game_menu,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                // randomize_washers,
                game_over,
                saturate_standard_material_alphas,
                // spawners
            )
                .run_if(in_state(IsPaused::Running)),
        )
        .observe(init_animations)
        .observe(init_animations_on_scene_instance);
    }
}

// fn spawners(
//     mut commands: Commands,
//     sensors: Query<
//         &CollidingEntities,
//         (With<Sensor>, With<Spawner>),
//     >,
//     players: Query<Entity, With<Player>>,
// ) {
//     for sensor in &sensors {
//         for player in &players {
//             if sensor.0.contains(&player) {
//                 let mut rng =
// rand::thread_rng();

//                 let x = rand::thread_rng()
//                     .gen_range(-10.0..10.0);
//                 let z = rand::thread_rng()
//                     .gen_range(-10.0..10.0);

//                 commands.spawn((
//                     Obstacle,
//                     BlueprintInfo::from_path(
//
// "blueprints/washing_machine.glb",
// ),                     SpawnBlueprint,
//
// TransformBundle::from_transform(
// Transform::from_xyz(x, 10.0, z)
// .with_rotation(
// Quat::from_rotation_z(
// rng.gen_range(0.0..PI),
// ),                             ),
//                     ),
//                     //
// CollisionGrouping::Enemy,
// RigidBody::Dynamic,                 ));
//             }
//         }
//     }
// }
fn game_over(
    mut commands: Commands,
    sensors: Query<
        &CollidingEntities,
        With<GameOverSensor>,
    >,
    machines: Query<Entity, With<WashingMachine>>,
    players: Query<Entity, With<Player>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for sensor in &sensors {
        for machine in &machines {
            if sensor.0.contains(&machine) {
                commands
                    .entity(machine)
                    .despawn_recursive();
            }
        }

        for player in &players {
            if sensor.0.contains(&player) {
                // commands.entity(player).
                // despawn_recursive();
                next_state.set(AppState::MainMenu);
            }
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct GameOverSensor;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct WashingMachine;

#[derive(Resource)]
struct DropTimer(Timer);

impl Default for DropTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(
            5.,
            TimerMode::Repeating,
        ))
    }
}

// fn randomize_washers(
//     mut commands: Commands,
//     mut timer: ResMut<DropTimer>,
//     time: Res<Time>,
// ) {
//     if timer.0.tick(time.delta()).
// just_finished() {         let mut rng =
// rand::thread_rng();

//         let x =
// rand::thread_rng().gen_range(-10.0..10.0);
//         let z =
// rand::thread_rng().gen_range(-10.0..10.0);

//         commands.spawn((
//             Obstacle,
//             BlueprintInfo::from_path(
//
// "blueprints/washing_machine.glb",
// ),             SpawnBlueprint,
//             TransformBundle::from_transform(
//                 Transform::from_xyz(x, 10.0, z)
//
// .with_rotation(Quat::from_rotation_z(
//                         rng.gen_range(0.0..PI),
//                     )),
//             ),
//             // CollisionGrouping::Enemy,
//             RigidBody::Dynamic,
//         ));
//     }
// }

#[derive(Component)]
pub struct Player;

fn setup_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // // Spawn the ground.
    // commands.spawn((
    //     PbrBundle {
    //         mesh: meshes.add(
    //             Plane3d::default()
    //                 .mesh()
    //                 .size(128.0, 128.0),
    //         ),
    //         material: materials.add(Color::WHITE),
    //         ..Default::default()
    //     },
    //     RigidBody::Static,
    //     Collider::half_space(Vec3::Y),
    // ));
    #[cfg(feature = "spawn_sacrifice")]
    commands.spawn((
        StateScoped(AppState::InGame),
        BlueprintInfo::from_path(
            "levels/Sacrifical Scene.glb",
        ),
        SpawnBlueprint,
        HideUntilReady,
        GameWorldTag,
    ));
    // #[cfg(not(feature = "spawn_sacrifice"))]
    // commands.spawn((
    //     StateScoped(AppState::InGame),
    //     BlueprintInfo::from_path("levels/level-001.
    // glb"),     SpawnBlueprint,
    //     HideUntilReady,
    //     GameWorldTag,
    // ));
    #[cfg(not(feature = "spawn_sacrifice"))]
    commands.spawn((
        StateScoped(AppState::InGame),
        BlueprintInfo::from_path("levels/level-002.glb"),
        SpawnBlueprint,
        HideUntilReady,
        GameWorldTag,
    ));

    // Spawn a little platform for the player to
    // jump on.
    // commands.spawn((
    //     PbrBundle {
    //         mesh: meshes.add(Cuboid::new(4.0,
    // 1.0, 4.0)),         material:
    // materials.add(Color::from(SLATE_200)),
    //         transform:
    // Transform::from_xyz(-6.0, 2.0, 0.0),
    //         ..Default::default()
    //     },
    //     RigidBody::Static,
    //     Collider::cuboid(4.0, 1.0, 4.0),
    // ));
}

#[derive(Component)]
pub struct PlayerMachineRangeSensor;

#[derive(Event)]
pub struct InvalidRangeToObject {
    pub object: Entity,
}

#[derive(Component)]
pub struct SaturateStandardMaterialAlpha(Timer);

fn saturate_standard_material_alphas(
    mut query: Query<(
        Entity,
        &mut SaturateStandardMaterialAlpha,
        &bevy::prelude::Handle<
            bevy::prelude::StandardMaterial,
        >,
        &mut Visibility,
    )>,
    mut materials: ResMut<
        Assets<bevy::prelude::StandardMaterial>,
    >,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut timer, handle, mut visibility) in
        &mut query
    {
        if timer.0.tick(time.delta()).just_finished() {
            // remove component, set color alpha to 1.
            commands
                .entity(entity)
                .remove::<SaturateStandardMaterialAlpha>();
            let mut mat =
                materials.get_mut(handle).unwrap();
            mat.base_color.set_alpha(1.);
            *visibility = Visibility::Hidden;
        } else {
            // set color alpha to timer percentage
            let mut mat =
                materials.get_mut(handle).unwrap();
            mat.base_color
                .set_alpha(timer.0.fraction_remaining());
        }
    }
}

fn spawn_player(
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    // mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 5_000.,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 2.0, 0.0),
                rotation: Quat::from_rotation_x(-1.7),
                ..default()
            },
            // The default cascade config is designed to
            // handle large scenes.
            // As this example has a much smaller world, we
            // can tighten the shadow bounds for
            // better visual quality.
            // cascade_shadow_config:
            //     CascadeShadowConfigBuilder {
            //         first_cascade_far_bound: 4.0,
            //         maximum_distance: 10.0,
            //         ..default()
            //     }
            //     .into(),
            ..default()
        },
        VolumetricLight,
        Name::new("DirectionalLight"),
    ));

    commands
        .spawn((
            Player,
            SceneBundle {
                scene: player_assets.player.clone(),
                transform: Transform::from_xyz(
                    0., 10., 0.,
                ),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::capsule(0.12, PLAYER_COLLIDER_HEIGHT),
            // This bundle holds the main components.
            TnuaControllerBundle::default(),
            // A sensor shape is not strictly necessary, but
            // without it we'll get weird results.
            TnuaAvian3dSensorShape(Collider::cylinder(
                0.11, 0.0,
            )),
            TnuaAnimatingState::<AnimationState>::default(),
            // Tnua can fix the rotation, but the character
            // will still get rotated before it can do so.
            // By locking the rotation we can prevent this.
            // LockedAxes::ROTATION_LOCKED,
            InputManagerBundle::with_map(
                PlayerAction::default_input_map(),
            ),
            Name::new("Player Scene"),
            CollisionLayers::new(
                GameLayer::Player,
                [GameLayer::Enemy, GameLayer::Ground],
            ),
            Inventory {
                max_item_count: 20,
                items: vec![],
            },
        ))
        .with_children(|builder| {
            let half_height = 0.05;
            let radius = 1.5;
            let player_half_height_guessed = 0.2;
            builder.spawn((
                PbrBundle {
                    mesh: meshes.add(Cylinder{
                        radius,
                        half_height: half_height,
                    }),
                    material: materials.add(StandardMaterial{
                        base_color: RED_400.into(),
                        alpha_mode: AlphaMode::Multiply,
                        ..default()
                    }),
                    transform: Transform::from_xyz(0., -player_half_height_guessed, 0.),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                Collider::cylinder(radius, half_height * 2.),
                Sensor,
                PlayerMachineRangeSensor,
                NotShadowCaster,
            ));
        }).observe(|
            trigger: Trigger<InvalidRangeToObject>,
            mut query: Query<(Entity, &mut Visibility), With<PlayerMachineRangeSensor>>,
            mut commands: Commands
        | {
            info!("Player tried to select something out of range");
            for (entity, mut visibility) in &mut query {
                *visibility = Visibility::Visible;
                commands.entity(entity).insert(SaturateStandardMaterialAlpha(Timer::from_seconds(0.2, TimerMode::Once)));
            }
        });
}

fn init_animations_on_scene_instance(
    trigger: Trigger<OnAdd, SceneInstance>,
) {
    info!("scene instance");
}

/// Attaches the animation graph to the scene, and
/// plays animations by weights.
fn init_animations(
    trigger: Trigger<OnAdd, AnimationPlayer>,
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &mut AnimationPlayer,
            &mut Transform,
        ),
        Without<ExampleAnimationWeights>,
    >,
    player_assets: Res<PlayerAssets>,
    gltfs: Res<Assets<Gltf>>,
    animation_clips: Res<Assets<AnimationClip>>,
    parent_query: Query<&Parent>,
    npcs: Query<&CustomerNpc>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    names: Query<&Name>,
    loaded_animation_graphs: Res<LoadedAnimationGraphs>,
) {
    for (entity, mut player, mut transform) in
        query.iter_mut()
    {
        let Some((animations, graph)) =
            names.get(entity).ok().and_then(|name| {
                loaded_animation_graphs.get(name.as_str())
            })
        else {
            continue;
        };

        // transform.scale = Vec3::splat(2.);
        transform.translation.y = -0.5;

        let mut trans = AnimationTransitions::new();
        trans
            .play(
                &mut player,
                CustomerNpcAnimationNames::Walk.into(),
                Duration::ZERO,
            )
            .repeat();
        commands.entity(entity).insert((
            trans,
            graph.clone(),
            animations.clone(),
        ));
    }
}

/// The indices of the nodes containing animation
/// clips in the graph.
static CLIP_NODE_INDICES: [u32; 4] = [1, 2, 3, 4];

/// The current weights of the three playing
/// animations.
#[derive(Debug, Component, Reflect)]
struct ExampleAnimationWeights {
    /// The weights of the three playing
    /// animations.
    weights: [f32; 4],
}

impl Default for ExampleAnimationWeights {
    fn default() -> Self {
        Self {
            weights: [0., 1., 0., 0.],
        }
    }
}
