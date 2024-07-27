use bevy::{
    color::palettes, gltf::GltfMesh, pbr::NotShadowCaster,
    prelude::*,
};
use rand::Rng;
use vleue_navigator::prelude::*;

use crate::{assets::NavMeshAssets, AppState};

pub struct NavMeshPlugin;

impl Plugin for NavMeshPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::InGame),
            setup_navmesh,
        )
        .add_plugins(VleueNavigatorPlugin)
        .add_systems(
            Update,
            (
                give_target_auto,
                trigger_navmesh_visibility,
                move_object,
            )
                .run_if(in_state(AppState::InGame)),
        );
    }
}

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
        let navmesh =
            vleue_navigator::NavMesh::from_bevy_mesh(
                meshes
                    .get(
                        &gltf_meshes
                            .get(navmesh)
                            .unwrap()
                            .primitives[0]
                            .mesh,
                    )
                    .unwrap(),
            );

        let mut material: StandardMaterial =
            Color::Srgba(palettes::css::ANTIQUE_WHITE)
                .into();
        material.unlit = true;

        let navmesh_wireframe =
            meshes.add(navmesh.to_wireframe_mesh());
        let navmesh_handle = navmeshes.add(navmesh);

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
