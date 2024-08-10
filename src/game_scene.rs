use avian3d::prelude::*;
use bevy::{
    color::palettes::tailwind::SLATE_200,
    core_pipeline::{
        bloom::BloomSettings,
        dof::{DepthOfFieldMode, DepthOfFieldSettings},
        tonemapping::Tonemapping,
    },
    math::vec3,
    pbr::{VolumetricFogSettings, VolumetricLight},
    prelude::*,
    render::primitives::Aabb,
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
use std::{f32::consts::PI, time::Duration};

use game_menu::spawn_game_menu;
use tnua_animation::{AnimationState, TnuaAnimationPlugin};

mod game_menu;
mod tnua_animation;

use crate::{
    assets::PlayerAssets,
    collision_layers::{CollisionGrouping, GameLayer},
    controls::PlayerAction,
    navmesh::{Obstacle, Spawner},
    states::{AppState, IsPaused},
};

pub struct GameScenePlugin;

impl Plugin for GameScenePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DropTimer>()
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
                (randomize_washers, game_over, spawners)
                    .run_if(in_state(IsPaused::Running)),
            )
            .observe(init_animations);
    }
}

fn spawners(
    mut commands: Commands,
    sensors: Query<
        &CollidingEntities,
        (With<Sensor>, With<Spawner>),
    >,
    players: Query<Entity, With<Player>>,
) {
    for sensor in &sensors {
        for player in &players {
            if sensor.0.contains(&player) {
                let mut rng = rand::thread_rng();

                let x = rand::thread_rng()
                    .gen_range(-10.0..10.0);
                let z = rand::thread_rng()
                    .gen_range(-10.0..10.0);

                commands.spawn((
                    Obstacle,
                    BlueprintInfo::from_path(
                        "blueprints/washing_machine.glb",
                    ),
                    SpawnBlueprint,
                    TransformBundle::from_transform(
                        Transform::from_xyz(x, 10.0, z)
                            .with_rotation(
                                Quat::from_rotation_z(
                                    rng.gen_range(0.0..PI),
                                ),
                            ),
                    ),
                    // CollisionGrouping::Enemy,
                    RigidBody::Dynamic,
                ));
            }
        }
    }
}
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
struct WashingMachine;

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

fn randomize_washers(
    mut commands: Commands,
    mut timer: ResMut<DropTimer>,
    time: Res<Time>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut rng = rand::thread_rng();

        let x = rand::thread_rng().gen_range(-10.0..10.0);
        let z = rand::thread_rng().gen_range(-10.0..10.0);

        commands.spawn((
            Obstacle,
            BlueprintInfo::from_path(
                "blueprints/washing_machine.glb",
            ),
            SpawnBlueprint,
            TransformBundle::from_transform(
                Transform::from_xyz(x, 10.0, z)
                    .with_rotation(Quat::from_rotation_z(
                        rng.gen_range(0.0..PI),
                    )),
            ),
            // CollisionGrouping::Enemy,
            RigidBody::Dynamic,
        ));
    }
}

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
    #[cfg(not(feature = "spawn_sacrifice"))]
    commands.spawn((
        StateScoped(AppState::InGame),
        BlueprintInfo::from_path("levels/level-001.glb"),
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

const AnimationNames: [&str; 32] = [
    "static",
    "idle",
    "walk",
    "sprint",
    "jump",
    "fall",
    "crouch",
    "sit",
    "drive",
    "die",
    "pick-up",
    "emote-yes",
    "emote-no",
    "holding-right",
    "holding-left",
    "holding-both",
    "holding-right-shoot",
    "holding-left-shoot",
    "holding-both-shoot",
    "attack-melee-right",
    "attack-melee-left",
    "attack-kick-right",
    "attack-kick-left",
    "interact-right",
    "interact-left",
    "wheelchair-sit",
    "wheelchair-look-left",
    "wheelchair-look-right",
    "wheelchair-move-forward",
    "wheelchair-move-back",
    "wheelchair-move-left",
    "wheelchair-move-right",
];
fn spawn_player(
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
    gltfs: Res<Assets<Gltf>>,
) {
    let Some(_player) = gltfs.get(&player_assets.gltf)
    else {
        warn!("player gltf should exist");
        return;
    };

    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 5_000.,
                ..default()
            },
            ..default()
        },
        VolumetricLight,
        Name::new("DirectionalLight"),
    ));

    commands.spawn((
        Player,
        SceneBundle {
            scene: player_assets.player.clone(),
            transform: Transform::from_xyz(0., 100., 0.),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::capsule(0.5, 1.),
        // This bundle holds the main components.
        TnuaControllerBundle::default(),
        // A sensor shape is not strictly necessary, but
        // without it we'll get weird results.
        TnuaAvian3dSensorShape(Collider::cylinder(
            0.49, 0.0,
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
    ));
}

/// Attaches the animation graph to the scene, and
/// plays animations by weights.
fn init_animations(
    trigger: Trigger<OnAdd, AnimationPlayer>,
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut AnimationPlayer,
        &mut Transform,
    )>,
    player_assets: Res<PlayerAssets>,
) {
    for (entity, mut player, mut transform) in
        query.iter_mut()
    {
        transform.scale = Vec3::splat(3.);
        commands.entity(entity).insert((
            player_assets.animation_graph.clone(),
            ExampleAnimationWeights::default(),
        ));
        for &node_index in &CLIP_NODE_INDICES {
            player.play(node_index.into()).repeat();
        }
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
