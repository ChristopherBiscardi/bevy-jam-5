use bevy::{
    core_pipeline::{
        bloom::BloomSettings,
        dof::{DepthOfFieldMode, DepthOfFieldSettings},
        tonemapping::Tonemapping,
    },
    pbr::VolumetricFogSettings,
    prelude::*,
    render::view::RenderLayers,
};
use bevy_mod_raycast::prelude::RaycastSource;

use crate::{
    game_scene::Player,
    states::{AppState, IsPaused},
};

pub struct WashCycleCameraPlugin;

impl Plugin for WashCycleCameraPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Dof>()
            .register_type::<GameCamera>()
            .insert_resource(Dof {
                focal_distance: 157.5,
                aperture_f_stops: 1.0 / 50.0,
                sensor_height: 0.01866,
                max_circle_of_confusion_diameter: 64.0,
                max_depth: f32::INFINITY,
            })
            .add_systems(
                OnEnter(AppState::InGame),
                spawn_3d_camera,
            )
            .add_systems(
                Update,
                (
                    dof_finder.run_if(in_state(
                        IsPaused::Running,
                    )),
                    dof_on_change
                        .run_if(resource_changed::<Dof>),
                ),
            )
            .add_systems(Startup, spawn_2d_camera);
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct GameCamera;

fn spawn_2d_camera(mut commands: Commands) {
    commands.spawn((
        RenderLayers::layer(1),
        Camera2dBundle {
            camera: Camera {
                order: 1,
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

fn spawn_3d_camera(mut commands: Commands, dof: Res<Dof>) {
    // commands.spawn(Camera3dBundle::default());
    commands.spawn((
        StateScoped(AppState::InGame),
        GameCamera,
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
