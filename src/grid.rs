use std::time::Duration;

use avian3d::prelude::CollidingEntities;
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

use crate::{
    camera::GameCamera,
    inventory::{Inventory, ProcessedState},
    game_scene::{
        InvalidRangeToObject, Player,
        PlayerMachineRangeSensor, WashingMachine,
    },
    states::{GameMode, IsPaused},
};

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
                Update,
                do_work.run_if(in_state(IsPaused::Running)),
            )
            .add_systems(
                OnEnter(GameMode::VirtualGridPlacement),
                spawn_virtual_placement_grid,
            )
            .add_systems(
                OnExit(GameMode::VirtualGridPlacement),
                exit_virtual_grid_placement,
            )
            .observe(test)
            .observe(interact_with_machine);
    }
}

#[derive(Resource, Debug, Default, Deref, DerefMut)]
struct GridStore(HashMap<IVec3, bool>);

#[derive(TypePath)]
struct VirtualGridRaycast;

fn exit_virtual_grid_placement(
    mut commands: Commands,
    current_cameras: Query<Entity, With<GameCamera>>,
) {
    for entity in &current_cameras {
        commands
            .entity(entity)
            .remove::<RaycastSource<VirtualGridRaycast>>();
    }
}

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
        StateScoped(GameMode::VirtualGridPlacement),
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

#[derive(Component)]
struct Working(Timer);

#[derive(Component)]
struct DefaultWorkDuration(Duration);

fn start_work(
    trigger: Trigger<StartWork>,
    mut commands: Commands,
    default_work_durations: Query<&DefaultWorkDuration>,
) {
    let Ok(duration) =
        default_work_durations.get(trigger.entity())
    else {
        warn!("DefaultWorkDuration component should exist on Machine");
        return;
    };
    commands.entity(trigger.entity()).insert(Working(
        Timer::new(duration.0, TimerMode::Once),
    ));
}

#[derive(Component, Debug)]
struct Done;

fn do_work(
    mut query: Query<(
        Entity,
        &mut Working,
        &mut Inventory,
    )>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut working, mut inventory) in &mut query {
        if working.0.tick(time.delta()).just_finished() {
            info!(?entity, "done");
            // TODO: Inventory Item state must change after
            // being worked on
            commands
                .entity(entity)
                .remove::<Working>()
                .insert(Done);
            for mut item in inventory.items.iter_mut() {
                item.process();
            }
        }
    }
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
                    Inventory{
                        max_item_count: 5,
                        items: vec![],
                    },
                    DefaultWorkDuration(Duration::from_secs(10))
                )).observe(start_work);

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

#[derive(Event)]
struct MachineInteract {
    machine_entity: Entity,
}

#[derive(Event)]
pub struct StartWork;

fn interact_with_machine(
    trigger: Trigger<MachineInteract>,
    mut machines: Query<
        (Entity, &mut Inventory, Option<&Done>),
        (With<WashingMachine>, Without<Working>),
    >,
    mut player: Query<
        (Entity, &mut Inventory),
        (With<Player>, Without<WashingMachine>),
    >,
    player_machine_sensor: Query<
        &CollidingEntities,
        With<PlayerMachineRangeSensor>,
    >,
    mut commands: Commands,
) {
    info!(
       trigger_entity=?trigger.entity(),
       event_entity=?trigger.event().machine_entity,
       "interact_with_machine"
    );
    for machine in &machines {
        info!(?machine, "a machine");
    }

    let Ok(player_sensor) =
        player_machine_sensor.get_single()
    else {
        warn!("expected exactly 1 player sensor");
        return;
    };

    let Ok((player_entity, mut player_inventory)) =
        player.get_single_mut()
    else {
        warn!("expected exactly 1 player");
        return;
    };

    dbg!(machines.get(trigger.event().machine_entity));
    let Ok((machine_entity, mut machine_inventory, done)) =
        machines.get_mut(trigger.event().machine_entity)
    else {
        warn!("expected exactly 1 machine");
        return;
    };

    // machines_done_with_work

    if player_sensor.contains(&machine_entity)
        && done.is_none()
    {
        // drop off into machine
        let available_space = machine_inventory
            .max_item_count
            - machine_inventory.items.len();

        let item_range = 0..(player_inventory
            .items
            .len()
            .min(available_space));

        let transition_items =
            player_inventory.items.drain(item_range);

        machine_inventory.items.extend(transition_items);
        commands.trigger_targets(StartWork, machine_entity);
    } else if player_sensor.contains(&machine_entity)
        && done.is_some()
    {
        // pickup from machine
        let available_space = player_inventory
            .max_item_count
            - player_inventory.items.len();

        let item_range = 0..(machine_inventory
            .items
            .len()
            .min(available_space));

        let transition_items =
            machine_inventory.items.drain(item_range);

        player_inventory.items.extend(transition_items);
        commands.entity(machine_entity).remove::<Done>();
    } else {
        // fire invalid machine choice by range
        commands.trigger_targets(
            InvalidRangeToObject {
                object: machine_entity,
            },
            player_entity,
        );
    }
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

                    match observer_name.as_str() {
                        "machine_interact" => {
                            world.commands().entity(entity).insert(
                                On::<Pointer<Click>>::run(
                                    move |mut commands: Commands| {
                                        info!(
                                            ?observer_name,
                                            "on click machine"
                                        );
                                        commands.trigger(
                                            MachineInteract {
                                                machine_entity: entity
                                            }
                                        );
                                    },
                                ),
                            );
                        }
                        observer => {
                            error!(?observer, "unhandled Observer defined in Blender");
                        }
                    }
             
            },
        );
    }
}
