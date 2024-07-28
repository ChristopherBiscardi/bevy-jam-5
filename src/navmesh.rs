use std::f32::consts::PI;

use bevy::{
    color::palettes,
    gltf::GltfMesh,
    math::{vec2, vec3},
    pbr::NotShadowCaster,
    prelude::*,
    render::primitives::Aabb,
};
use blenvy::{BlueprintInfo, SpawnBlueprint};
use geo::{LineString, Polygon as GeoPolygon};
use rand::{rngs::ThreadRng, Rng};
use vleue_navigator::{prelude::*, Triangulation};

use crate::{assets::NavMeshAssets, AppState};

pub struct NavMeshPlugin;

impl Plugin for NavMeshPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            VleueNavigatorPlugin,
            NavmeshUpdaterPlugin::<Aabb, Obstacle>::default(
            ),
        ))
        .register_type::<Obstacle>()
        .add_systems(
            OnEnter(AppState::InGame),
            setup_navmesh,
        )
        .add_systems(
            Update,
            (
                give_target_auto,
                trigger_navmesh_visibility,
                move_object,
                spawn_obstacle_on_click,
                debug,
            )
                .run_if(in_state(AppState::InGame)),
        );
    }
}

fn debug(
    navmeshes: Res<Assets<NavMesh>>,
    current_mesh: Res<CurrentMesh>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut handles: Query<
        &mut Handle<Mesh>,
        With<NavMeshDisp>,
    >,
) {
    // info!("navmeshes.len: {}", navmeshes.len());
    let navmesh = navmeshes.get(&current_mesh.0).unwrap();

    let navmesh_wireframe =
        meshes.add(navmesh.to_wireframe_mesh());
    for mut handle in &mut handles {
        *handle = navmesh_wireframe.clone();
    }
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
struct Obstacle;

#[derive(Resource)]
struct CurrentMesh(Handle<NavMesh>);

#[derive(Component, Clone)]
struct NavMeshDisp(Handle<NavMesh>);

#[derive(Component)]
struct Object(Option<Entity>);

#[derive(Component)]
struct Target;

#[derive(Component)]
struct Path {
    current: Vec3,
    next: Vec<Vec3>,
}

// pub fn from_outer_edges(edges: &[Vec2]) ->
// Triangulation {     Triangulation {
//         inner: GeoPolygon::new(
//             LineString::from(
//                 edges
//                     .iter()
//                     .map(|v| (v.x, v.y))
//                     .collect::<Vec<_>>(),
//             ),
//             vec![],
//         ),
//         prebuilt: None,
//     }
// }

fn setup_navmesh(
    mut commands: Commands,
    navmesh_assets: Res<NavMeshAssets>,
    gltfs: Res<Assets<Gltf>>,
    gltf_meshes: Res<Assets<GltfMesh>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut navmeshes: ResMut<Assets<NavMesh>>,
) {
    let Some(gltf) =
        gltfs.get(&navmesh_assets.navmesh_gltf)
    else {
        warn!("navmesh gltf missing!");
        return;
    };

    let Some(navmesh) = gltf.named_meshes.get("navmesh")
    else {
        let available_mesh_names: Vec<_> =
            gltf.named_meshes.keys().collect();
        warn!(
            ?available_mesh_names,
            "navmesh missing!"
        );
        return;
    };

    {
        let mesh = meshes
            .get(
                &gltf_meshes
                    .get(navmesh)
                    .unwrap()
                    .primitives[0]
                    .mesh,
            )
            .unwrap();
        let aabb = mesh.compute_aabb().unwrap();
        let lower = aabb.center - aabb.half_extents;
        let higher = aabb.center + aabb.half_extents;

        let navmesh =
            vleue_navigator::NavMesh::from_bevy_mesh(mesh);

        let mut material: StandardMaterial =
            Color::Srgba(palettes::css::ANTIQUE_WHITE)
                .into();
        material.unlit = true;

        let navmesh_wireframe =
            meshes.add(navmesh.to_wireframe_mesh());
        let navmesh_handle = navmeshes.add(navmesh);

        // commands.spawn(NavMeshBundle {
        //     settings: NavMeshSettings {
        //         // Define the outer borders of the
        // navmesh.         fixed:
        // Triangulation::from_outer_edges(
        //             &vec![
        //                 vec2(
        //                     -(aabb.half_extents.x *
        // 20.),
        // -(aabb.half_extents.z * 20.),
        //                 ),
        //                 vec2(
        //                     -(aabb.half_extents.x *
        // 20.),
        // (aabb.half_extents.z * 20.),
        //                 ),
        //                 vec2(
        //                     (aabb.half_extents.x *
        // 20.),
        // (aabb.half_extents.z * 20.),
        //                 ),
        //                 vec2(
        //                     (aabb.half_extents.x *
        // 20.),
        // -(aabb.half_extents.z * 20.),
        //                 ),
        //             ],
        //         ),
        //         ..default()
        //     },
        //     update_mode: NavMeshUpdateMode::Direct,
        //     handle: navmesh_handle.clone(),
        //     ..default()
        // });
        commands.insert_resource(CurrentMesh(
            navmesh_handle.clone(),
        ));
        commands.spawn((
            PbrBundle {
                mesh: navmesh_wireframe,
                material: materials.add(material),
                transform: Transform::from_xyz(
                    0.0, 0.2, 0.0,
                ),
                visibility: Visibility::Hidden,
                ..Default::default()
            },
            NavMeshDisp(navmesh_handle),
        ));
    }

    commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(Capsule3d {
                    ..default()
                })),
                material: materials.add(StandardMaterial {
                    base_color: palettes::css::BLUE.into(),
                    emissive: (palettes::css::BLUE * 5.0)
                        .into(),
                    ..default()
                }),
                transform: Transform::from_xyz(5., 1.0, 5.),
                ..Default::default()
            },
            Object(None),
            NotShadowCaster,
        ))
        .with_children(|object| {
            object.spawn(PointLightBundle {
                point_light: PointLight {
                    color: palettes::css::BLUE.into(),
                    range: 500.0,
                    intensity: 100000.0,
                    shadows_enabled: true,
                    ..default()
                },
                transform: Transform::from_xyz(
                    0.0, 1.2, 0.0,
                ),
                ..default()
            });
        });
}

fn give_target_auto(
    mut commands: Commands,
    mut object_query: Query<
        (Entity, &Transform, &mut Object),
        Without<Path>,
    >,
    navmeshes: Res<Assets<NavMesh>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    current_mesh: Res<CurrentMesh>,
) {
    for (entity, transform, mut object) in
        object_query.iter_mut()
    {
        let navmesh =
            navmeshes.get(&current_mesh.0).unwrap();
        let mut x;
        let mut z;
        loop {
            x = rand::thread_rng().gen_range(-10.0..10.0);
            z = rand::thread_rng().gen_range(-10.0..10.0);

            if navmesh.transformed_is_in_mesh(Vec3::new(
                x, 0.0, z,
            )) {
                break;
            }
        }

        let Some(path) = navmesh.transformed_path(
            transform.translation,
            Vec3::new(x, 0.0, z),
        ) else {
            break;
        };
        if let Some((first, remaining)) =
            path.path.split_first()
        {
            let mut remaining = remaining.to_vec();
            remaining.reverse();
            let target_id = commands
                .spawn((
                    PbrBundle {
                        mesh: meshes.add(Mesh::from(
                            Sphere {
                                radius: 0.5,
                                ..default()
                            },
                        )),
                        material: materials.add(
                            StandardMaterial {
                                base_color:
                                    palettes::css::RED
                                        .into(),
                                emissive:
                                    (palettes::css::RED
                                        * 5.0)
                                        .into(),
                                ..default()
                            },
                        ),
                        transform: Transform::from_xyz(
                            x, 0.0, z,
                        ),
                        ..Default::default()
                    },
                    NotShadowCaster,
                    Target,
                ))
                .with_children(|target| {
                    target.spawn(PointLightBundle {
                        point_light: PointLight {
                            color: palettes::css::RED
                                .into(),
                            shadows_enabled: true,
                            range: 10.0,
                            ..default()
                        },
                        transform: Transform::from_xyz(
                            0.0, 1.5, 0.0,
                        ),
                        ..default()
                    });
                })
                .id();
            commands.entity(entity).insert(Path {
                current: first.clone(),
                next: remaining,
            });
            object.0 = Some(target_id);
        }
    }
}

fn trigger_navmesh_visibility(
    mut query: Query<(&mut Visibility, &NavMeshDisp)>,
    keyboard_input: ResMut<ButtonInput<KeyCode>>,
    current_mesh: Res<CurrentMesh>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for (mut visible, nav) in query.iter_mut() {
            if nav.0 == current_mesh.0 {
                match *visible {
                    Visibility::Visible => {
                        *visible = Visibility::Hidden
                    }
                    Visibility::Hidden => {
                        *visible = Visibility::Visible
                    }
                    Visibility::Inherited => {
                        *visible = Visibility::Inherited
                    }
                }
            }
        }
    }
}

fn move_object(
    mut commands: Commands,
    mut object_query: Query<(
        &mut Transform,
        &mut Path,
        Entity,
        &mut Object,
    )>,
    time: Res<Time>,
) {
    for (mut transform, mut target, entity, mut object) in
        object_query.iter_mut()
    {
        let move_direction =
            target.current - transform.translation;
        transform.translation += move_direction.normalize()
            * time.delta_seconds()
            * 10.0;
        if transform.translation.distance(target.current)
            < 0.1
        {
            if let Some(next) = target.next.pop() {
                target.current = next;
            } else {
                commands.entity(entity).remove::<Path>();
                let target_entity =
                    object.0.take().unwrap();
                commands
                    .entity(target_entity)
                    .despawn_recursive();
            }
        }
    }
}

fn spawn_obstacle_on_click(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut commands: Commands,
    settings: Query<Ref<NavMeshSettings>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let mut rng = rand::thread_rng();
        let x = rand::thread_rng().gen_range(-10.0..10.0);
        let z = rand::thread_rng().gen_range(-10.0..10.0);

        // new_obstacle(
        //     &mut commands,
        //     &mut rng,
        //     Transform::from_xyz(x, 0.0, z),
        // );
        commands.spawn((
            Obstacle,
            Aabb::from_min_max(
                vec3(0., 0., 0.),
                vec3(1., 1., 1.),
            ),
            BlueprintInfo::from_path(
                "blueprints/washing_machine.glb",
            ),
            SpawnBlueprint,
            TransformBundle::from_transform(
                Transform::from_xyz(x, 0.0, z)
                    .with_rotation(Quat::from_rotation_z(
                        rng.gen_range(0.0..PI),
                    )),
            ),
        ));
    }
}
// Aabb::from_min_max(
//     Vec3::ZERO,
//     Vec3::new(
//         rng.gen_range(0.1..0.5),
//         rng.gen_range(0.1..0.5),
//         0.0,
//     ),
// ),

fn new_obstacle(
    commands: &mut Commands,
    rng: &mut ThreadRng,
    transform: Transform,
) {
    commands.spawn((
        match rng.gen_range(0..8) {
            0 => PrimitiveObstacle::Rectangle(Rectangle {
                half_size: vec2(
                    rng.gen_range(1.0..5.0),
                    rng.gen_range(1.0..5.0),
                ),
            }),
            1 => PrimitiveObstacle::Circle(Circle {
                radius: rng.gen_range(1.0..5.0),
            }),
            2 => PrimitiveObstacle::Ellipse(Ellipse {
                half_size: vec2(
                    rng.gen_range(1.0..5.0),
                    rng.gen_range(1.0..5.0),
                ),
            }),
            3 => PrimitiveObstacle::CircularSector(
                CircularSector::new(
                    rng.gen_range(1.5..5.0),
                    rng.gen_range(0.5..PI),
                ),
            ),
            4 => PrimitiveObstacle::CircularSegment(
                CircularSegment::new(
                    rng.gen_range(1.5..5.0),
                    rng.gen_range(1.0..PI),
                ),
            ),
            5 => {
                PrimitiveObstacle::Capsule(Capsule2d::new(
                    rng.gen_range(1.0..3.0),
                    rng.gen_range(1.5..5.0),
                ))
            }
            6 => PrimitiveObstacle::RegularPolygon(
                RegularPolygon::new(
                    rng.gen_range(1.0..5.0),
                    rng.gen_range(3..8),
                ),
            ),
            7 => PrimitiveObstacle::Rhombus(Rhombus::new(
                rng.gen_range(3.0..6.0),
                rng.gen_range(2.0..3.0),
            )),
            _ => unreachable!(),
        },
        transform,
        GlobalTransform::default(),
    ));
}
