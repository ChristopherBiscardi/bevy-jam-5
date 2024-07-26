use avian3d::prelude::*;
use bevy::{
    color::palettes::tailwind::SLATE_200,
    core_pipeline::{
        bloom::BloomSettings,
        dof::{DepthOfFieldMode, DepthOfFieldSettings},
        tonemapping::Tonemapping,
    },
    pbr::{VolumetricFogSettings, VolumetricLight},
    prelude::*,
};

use bevy_tnua::{prelude::*, TnuaAnimatingState};
use bevy_tnua_avian3d::*;
use blenvy::{
    BlueprintInfo, GameWorldTag, HideUntilReady,
    SpawnBlueprint,
};
use tnua_animation::{AnimationState, TnuaAnimationPlugin};
mod tnua_animation;

use crate::{
    assets::PlayerAssets, collision_layers::GameLayer,
    AppState, Dof, GameRenderLayer,
};

pub struct GameScenePlugin;

impl Plugin for GameScenePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ExampleAnimationWeights>()
            .add_plugins((
                PhysicsPlugins::default(),
                PhysicsDebugPlugin::default(),
                // We need both Tnua's main controller
                // plugin, and the plugin to connect to the
                // physics backend (in this
                // case XBPD-3D)
                TnuaControllerPlugin::default(),
                TnuaAvian3dPlugin::default(),
                TnuaAnimationPlugin,
            ))
            .add_systems(
                OnEnter(AppState::InGame),
                (
                    spawn_3d_camera,
                    spawn_player,
                    setup_level,
                    // spawn_the_cube,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                (sync_weights, spawn_the_cube),
            )
            .add_systems(
                Update,
                (apply_controls
                    .in_set(TnuaUserControlsSystemSet),),
            )
            .observe(init_animations);
    }
}

fn spawn_the_cube(
    mut commands: Commands,
    keycode: Res<ButtonInput<KeyCode>>,
) {
    if keycode.just_pressed(KeyCode::KeyS) {
        commands.spawn((
            BlueprintInfo::from_path(
                "blueprints/washing_machine.glb",
            ),
            SpawnBlueprint,
            TransformBundle::from_transform(
                Transform::from_xyz(0., 2., 0.),
            ),
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

    commands.spawn((
        BlueprintInfo::from_path("levels/Scene.glb"),
        SpawnBlueprint,
        HideUntilReady,
        GameWorldTag,
    ));

    // Spawn a little platform for the player to jump
    // on.
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(4.0, 1.0, 4.0)),
            material: materials.add(Color::from(SLATE_200)),
            transform: Transform::from_xyz(-6.0, 2.0, 0.0),
            ..Default::default()
        },
        RigidBody::Static,
        Collider::cuboid(4.0, 1.0, 4.0),
    ));
}

fn apply_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut TnuaController>,
) {
    let Ok(mut controller) = query.get_single_mut() else {
        return;
    };

    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::ArrowUp) {
        direction -= Vec3::Z;
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        direction += Vec3::Z;
    }
    if keyboard.pressed(KeyCode::ArrowLeft) {
        direction -= Vec3::X;
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        direction += Vec3::X;
    }

    // Feed the basis every frame. Even if the player
    // doesn't move - just use `desired_velocity:
    // Vec3::ZERO`. `TnuaController` starts without a
    // basis, which will make the character collider
    // just fall.
    controller.basis(TnuaBuiltinWalk {
        // The `desired_velocity` determines how the
        // character will move.
        desired_velocity: direction.normalize_or_zero()
            * 10.0,
        // The `float_height` must be greater (even if by
        // little) from the distance between the
        // character's center and the lowest point of its
        // collider.
        float_height: 1.5,
        // `TnuaBuiltinWalk` has many other fields for
        // customizing the movement - but they have
        // sensible defaults. Refer to the
        // `TnuaBuiltinWalk`'s documentation to learn what
        // they do.
        ..Default::default()
    });

    // Feed the jump action every frame as long as the
    // player holds the jump button. If the player
    // stops holding the jump button, simply stop
    // feeding the action.
    if keyboard.pressed(KeyCode::Space) {
        controller.action(TnuaBuiltinJump {
            // The height is the only mandatory field of the
            // jump button.
            height: 4.0,
            // `TnuaBuiltinJump` also has customization
            // fields with sensible defaults.
            ..Default::default()
        });
    }
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
        LockedAxes::ROTATION_LOCKED,
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
/// Takes the weights that were set in the UI and
/// assigns them to the actual playing animation.
fn sync_weights(
    mut query: Query<(
        &mut AnimationPlayer,
        &ExampleAnimationWeights,
    )>,
) {
    // for (mut animation_player,
    // animation_weights) in
    //     query.iter_mut()
    // {
    //     for (&animation_node_index,
    // &animation_weight) in
    //         CLIP_NODE_INDICES
    //             .iter()
    //             
    // .zip(animation_weights.weights.iter())
    //     {
    //         // If the animation happens to be
    // no longer active, restart it.
    //         if
    // !animation_player.animation_is_playing(
    //             animation_node_index.into(),
    //         ) {
    //             animation_player
    //                 
    // .play(animation_node_index.into())
    //                 .repeat();
    //         }

    //         // Set the weight.
    //         if let Some(active_animation) =
    // animation_player             
    // .animation_mut(animation_node_index.into())
    //         {
    //             active_animation
    //                 
    // .set_weight(animation_weight);
    //         }
    //     }
    // }
}

fn spawn_3d_camera(mut commands: Commands, dof: Res<Dof>) {
    // commands.spawn(Camera3dBundle::default());
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(83., 92., 100.0)
                .with_rotation(Quat::from_euler(
                    EulerRot::XYZ,
                    -0.78,
                    0.61,
                    0.5,
                )),
            projection: Projection::Perspective(
                PerspectiveProjection {
                    fov: 0.2,
                    ..default()
                },
            ),
            // .looking_at(Vec3::new(0.0, 0.3, 0.0),
            // Vec3::Y), projection:
            // Projection::Orthographic(
            //     OrthographicProjection {
            //         far: 1000.,
            //         near: -1000.,
            //         scale: 0.03,
            //         ..Default::default()
            //     },
            // ),
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            ..default()
        },
        BloomSettings::NATURAL,
        DepthOfFieldSettings {
            // Bokeh only works on Native
            mode: DepthOfFieldMode::Bokeh,
            focal_distance: dof.focal_distance,
            aperture_f_stops: dof.aperture_f_stops,
            sensor_height: dof.sensor_height,
            max_circle_of_confusion_diameter: dof
                .max_circle_of_confusion_diameter,
            max_depth: dof.max_depth,
            // focal_distance: dof.            //
            // focal_distance,

            // // Set a nice blur level.
            // //
            // // This is a really low F-number, but we want
            // to demonstrate the // effect,
            // even if it's kind of unrealistic.
            // aperture_f_stops: 1.0 / 50.0,
            // max_depth: 14.0,
            // ..default()
        },
        VolumetricFogSettings {
            // This value is explicitly set to 0 since we
            // have no environment map light
            ambient_intensity: 0.0,
            ..default()
        },
    ));

    // commands.spawn((Camera3dBundle {
    //     transform: Transform::from_xyz(0., 10.,
    // 30.)         .looking_at(Vec3::new(0.,
    // 0.5, 0.), Vec3::Y),     projection:
    // Projection::Perspective(
    //         PerspectiveProjection {
    //             fov: 0.5,
    //             ..default()
    //         },
    //     ),
    //     camera: Camera {
    //         hdr: true,
    //         ..default()
    //     },
    //     tonemapping:
    // Tonemapping::TonyMcMapface,
    //     ..default()
    // },));
}
