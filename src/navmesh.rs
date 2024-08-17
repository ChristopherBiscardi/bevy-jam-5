use std::f32::consts::PI;

use avian3d::prelude::{Collider, Sensor};
use bevy::{
    color::palettes::{
        self,
        tailwind::{GREEN_400, RED_400},
    },
    gltf::GltfMesh,
    math::{vec2, vec3},
    pbr::NotShadowCaster,
    prelude::*,
    render::primitives::Aabb,
};
use blenvy::{BlueprintInfo, SpawnBlueprint};
use geo::{LineString, Polygon as GeoPolygon};
use rand::{rngs::ThreadRng, Rng};
use vleue_navigator::{
    prelude::*, NavMeshDebug, Triangulation,
};

use crate::{
    assets::NavMeshAssets,
    customer_npc::CustomerNpc,
    states::{AppState, IsPaused},
};

pub struct NavMeshPlugin;

impl Plugin for NavMeshPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NavMeshesDebug(
            palettes::tailwind::PINK_400.into(),
        ))
        .add_plugins((
            VleueNavigatorPlugin,
            NavmeshUpdaterPlugin::<Collider, Obstacle>::default(
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
            )
                .run_if(in_state(IsPaused::Running)),
        )
        .observe(spawn_obstacle);
    }
}

#[derive(Component)]
pub struct Spawner;

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Obstacle;

#[derive(Resource)]
struct CurrentMesh(Handle<NavMesh>);

#[derive(Component, Clone)]
struct NavMeshDisp(Handle<NavMesh>);

#[derive(Component)]
pub struct Object(pub Option<Entity>);

#[derive(Component)]
struct Target;

#[derive(Component)]
pub struct Path {
    pub current: Vec3,
    pub next: Vec<Vec3>,
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

        let transform = navmesh.transform();
        let mut material: StandardMaterial =
            Color::Srgba(palettes::css::ANTIQUE_WHITE)
                .into();
        material.unlit = true;

        let navmesh_handle = navmeshes.add(navmesh);

        commands.spawn(NavMeshBundle {
            settings: NavMeshSettings {
                // Define the outer borders of the
                // navmesh.
                fixed: Triangulation::from_outer_edges(
                    &vec![
                        vec2(
                            -(aabb.half_extents.x),
                            -(aabb.half_extents.z),
                        ),
                        vec2(
                            -(aabb.half_extents.x),
                            (aabb.half_extents.z),
                        ),
                        vec2(
                            (aabb.half_extents.x),
                            (aabb.half_extents.z),
                        ),
                        vec2(
                            (aabb.half_extents.x),
                            -(aabb.half_extents.z),
                        ),
                    ],
                ),
                ..default()
            },
            update_mode: NavMeshUpdateMode::Direct,
            handle: navmesh_handle.clone(),
            transform: transform,
            ..default()
        });
        commands.insert_resource(CurrentMesh(
            navmesh_handle.clone(),
        ));
    }

    commands
        .spawn((
            Spawner,
            Sensor,
            Collider::capsule(1., 1.),
            PbrBundle {
                mesh: meshes.add(Mesh::from(Capsule3d {
                    ..default()
                })),
                material: materials.add(StandardMaterial {
                    base_color: RED_400.into(),
                    emissive: (RED_400 * 5.0).into(),
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
                    color: RED_400.into(),
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
        (Without<Path>, Without<CustomerNpc>),
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
                                base_color: GREEN_400
                                    .into(),
                                emissive: (GREEN_400 * 5.0)
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
                            color: GREEN_400.into(),
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
    mut object_query: Query<
        (
            &mut Transform,
            &mut Path,
            Entity,
            &mut Object,
        ),
        Without<CustomerNpc>,
    >,
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

#[derive(Event)]
pub struct SpawnObstacle;

fn spawn_obstacle(
    trigger: Trigger<SpawnObstacle>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut commands: Commands,
    settings: Query<Ref<NavMeshSettings>>,
) {
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
            Transform::from_xyz(x, 0.0, z).with_rotation(
                Quat::from_rotation_z(
                    rng.gen_range(0.0..PI),
                ),
            ),
        ),
    ));
}
