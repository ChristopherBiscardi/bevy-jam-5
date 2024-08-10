use bevy::{
    color::palettes::tailwind::PINK_400,
    ecs::{
        component::{
            ComponentHooks, ComponentId, StorageType,
        },
        world::DeferredWorld,
    },
    math::{ivec3, vec3},
    prelude::*,
    utils::HashMap,
};
use bevy_mod_picking::{
    events::{Click, Pointer},
    prelude::On,
};
use bevy_mod_raycast::prelude::*;

use crate::{camera::GameCamera, states::GameMode};

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<BlenderOnClick>()
            .init_resource::<GridStore>()
            .add_plugins(DeferredRaycastingPlugin::<
                VirtualGridRaycast,
            >::default())
            .add_systems(
                Update,
                raycast_system.run_if(in_state(
                    GameMode::VirtualGridPlacement,
                )),
            )
            .add_systems(
                OnEnter(GameMode::VirtualGridPlacement),
                spawn_virtual_placement_grid,
            )
            .observe(test);
    }
}

#[derive(Resource, Debug, Default, Deref, DerefMut)]
struct GridStore(HashMap<IVec3, bool>);

#[derive(TypePath)]
struct VirtualGridRaycast;

fn spawn_virtual_placement_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    current_camera: Query<Entity, With<GameCamera>>,
) {
    let Ok(entity) = current_camera.get_single() else {
        error!("Wrong number of cameras");
        return;
    };
    commands.entity(entity).insert(RaycastSource::<
        VirtualGridRaycast,
    >::new_cursor());

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(
                Plane3d::default()
                    .mesh()
                    .size(50.0, 50.0)
                    .subdivisions(10),
            ),
            material: materials.add(Color::from(
                bevy::color::palettes::tailwind::GREEN_400,
            )),
            ..default()
        },
        RaycastMesh::<VirtualGridRaycast>::default(),
    ));
}

fn raycast_system(
    mut commands: Commands,
    mut gizmos: Gizmos,
    query: Query<&RaycastMesh<VirtualGridRaycast>>,
    input: Res<ButtonInput<MouseButton>>,
    mut grid_store: ResMut<GridStore>,
) {
    for raycast_mesh in &query {
        for (entity, intersection_data) in
            raycast_mesh.intersections()
        {
            let pos = intersection_data.position().round();
            gizmos.cuboid(
                Transform::from_translation(pos),
                PINK_400,
            );
            if input.just_pressed(MouseButton::Left) {
                if grid_store.get(&pos.as_ivec3()).is_none()
                {
                    commands.spawn((
                    crate::navmesh::Obstacle,
                    blenvy::BlueprintInfo::from_path(
                        "blueprints/washing_machine.glb",
                    ),
                    blenvy::SpawnBlueprint,
                    TransformBundle::from_transform(
                        Transform::from_translation(pos),
                    ),
                ));
                    grid_store.insert(pos.as_ivec3(), true);
                } else if grid_store
                    .get(&(pos.as_ivec3() + ivec3(0, 1, 0)))
                    .is_none()
                {
                    let pos =
                        pos.as_ivec3() + ivec3(0, 1, 0);
                    commands.spawn((
                            crate::navmesh::Obstacle,
                            blenvy::BlueprintInfo::from_path(
                                "blueprints/washing_machine.glb",
                            ),
                            blenvy::SpawnBlueprint,
                            TransformBundle::from_transform(
                                Transform::from_translation(pos.as_vec3()),
                            ),
                        ));
                } else {
                    info!("blocked");
                }
            }
        }
    }
}

fn test(
    trigger: Trigger<BlenderOnClick>,
    mut local: Local<u8>,
) {
    info!(?local, "old");
    *local += 1;
}

#[derive(Reflect, Debug)]
#[reflect(Component)]
struct BlenderOnClick {
    observer_name: String,
}

impl Event for BlenderOnClick {}

impl Component for BlenderOnClick {
    const STORAGE_TYPE: StorageType =
        StorageType::SparseSet;

    fn register_component_hooks(
        hooks: &mut ComponentHooks,
    ) {
        hooks.on_add(
            |mut world: DeferredWorld,
             entity: Entity,
             _id: ComponentId| {
                let observer_name = world
                    .get::<BlenderOnClick>(entity)
                    .unwrap()
                    .observer_name
                    .clone();

                world.commands().entity(entity).insert(
                    On::<Pointer<Click>>::run(
                        move |mut commands: Commands| {
                            info!(
                                ?observer_name,
                                "on click"
                            );
                            commands.trigger(
                                BlenderOnClick {
                                    observer_name: "test"
                                        .to_string(),
                                },
                            );
                        },
                    ),
                );
            },
        );
    }
}
