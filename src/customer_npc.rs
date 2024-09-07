use avian3d::prelude::{
    Collider, CollidingEntities, CollisionStarted,
};
use bevy::{
    color::palettes::tailwind::*,
    prelude::*,
    render::{
        mesh::VertexAttributeValues,
        render_asset::RenderAssetUsages,
        render_resource::PrimitiveTopology,
        view::RenderLayers,
    },
    utils::HashSet,
};
use bevy_mod_picking::prelude::*;
use rand::{seq::IteratorRandom, Rng};
use std::{
    ops::Deref,
    time::{Duration, Instant},
};
use vello::wgpu::{
    Extent3d, TextureDimension, TextureFormat,
};
use woodpecker_ui::prelude::*;

use crate::{
    assets::{FontAssets, FontVelloAssets, PlayerAssets},
    game_scene::Player,
    inventory::{Inventory, Item, ProcessedState},
    navmesh::{Object, Path, SpawnObstacle},
    persistent_id::PersistentId,
    states::{AppState, GameMode, IsPaused},
    widgets::{self, *},
};

pub struct CustomerNpcPlugin;

impl Plugin for CustomerNpcPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CustomerNpcSpawner>()
            .register_type::<CustomerDropoffLocation>()
            .register_type::<PlayerReceiveFromCustomerLocation>()
            .register_type::<TheLight>()
            .add_systems(
                Update,
                (
                    move_customer,
                    detect_customer_dropoff,
                    detect_pickup,
                    detect_player_return_to_customer_pickup,
                )
                    .run_if(in_state(IsPaused::Running)),
            )
            .add_systems(
                FixedUpdate,
                customer_spawn_cycle
                    .run_if(in_state(IsPaused::Running))
            )
            .observe(spawn_customer_npc);
    }
}

// const CUSTOMER_NPC_ANIMATION_NAMES: [&str; 32] = [
//     "static",
//     "idle",
//     "walk",
//     "sprint",
//     "jump",
//     "fall",
//     "crouch",
//     "sit",
//     "drive",
//     "die",
//     "pick-up",
//     "emote-yes",
//     "emote-no",
//     "holding-right",
//     "holding-left",
//     "holding-both",
//     "holding-right-shoot",
//     "holding-left-shoot",
//     "holding-both-shoot",
//     "attack-melee-right",
//     "attack-melee-left",
//     "attack-kick-right",
//     "attack-kick-left",
//     "interact-right",
//     "interact-left",
//     "wheelchair-sit",
//     "wheelchair-look-left",
//     "wheelchair-look-right",
//     "wheelchair-move-forward",
//     "wheelchair-move-back",
//     "wheelchair-move-left",
//     "wheelchair-move-right",
// ];

pub enum CustomerNpcAnimationNames {
    Static = 1,
    Idle,
    Walk,
    Sprint,
    Jump,
    Fall,
    Crouch,
    Sit,
    Drive,
    Die,
    PickUp,
    EmoteYes,
    EmoteNo,
    HoldingRight,
    HoldingLeft,
    HoldingBoth,
    HoldingRightShoot,
    HoldingLeftShoot,
    HoldingBothShoot,
    AttackMeleeRight,
    AttackMeleeLeft,
    AttackKickRight,
    AttackKickLeft,
    InteractRight,
    InteractLeft,
    WheelchairSit,
    WheelchairLookLeft,
    WheelchairLookRight,
    WheelchairMoveForward,
    WheelchairMoveBack,
    WheelchairMoveLeft,
    WheelchairMoveRight,
}

impl From<CustomerNpcAnimationNames>
    for AnimationNodeIndex
{
    fn from(value: CustomerNpcAnimationNames) -> Self {
        (value as u32).into()
    }
}

#[derive(Component)]
pub struct CustomerNpc {
    pub gltf: Handle<Gltf>,
    // TODO: when Items become Entitys, remove this field.
    pub expected_number_items_to_leave: usize,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct TheLight;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct CustomerNpcSpawner;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct CustomerDropoffLocation;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct PlayerReceiveFromCustomerLocation;

#[derive(Event)]
pub struct CustomerNpcSpawnEvent;

fn spawn_customer_npc(
    trigger: Trigger<CustomerNpcSpawnEvent>,
    spawners: Query<
        (Entity, &Handle<Mesh>, &Parent),
        With<CustomerNpcSpawner>,
    >,
    transforms: Query<&Transform>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    dropoff_locations: Query<
        (Entity, &Transform),
        With<CustomerDropoffLocation>,
    >,
    player_assets: Res<PlayerAssets>,
    gltfs: Res<Assets<Gltf>>,
) {
    // TODO: bevy 0.15: UniformMeshSampling is now a
    // thing, we can remove this Rectangle
    // assumption code
    let Ok((spawner_entity, spawner_mesh_handle, parent)) =
        spawners.get_single()
    else {
        warn!("only expected one spawner");
        return;
    };

    let Some(spawner_mesh) =
        meshes.get(spawner_mesh_handle)
    else {
        warn!("no available mesh");
        return;
    };

    if !matches!(
        spawner_mesh.primitive_topology(),
        PrimitiveTopology::TriangleList
    ) {
        warn!("Spawner is not a TriangleList");
        return;
    }
    // dbg!(spawner_mesh);
    // let Some(indices) = spawner_mesh.indices() else
    // {     warn!("indices unavailable");
    //     return;
    // };
    let Some(VertexAttributeValues::Float32x3(positions)) =
        spawner_mesh.attribute(Mesh::ATTRIBUTE_POSITION)
    else {
        warn!("can't get ATTRIBUTE_POSITION from Mesh");
        return;
    };

    if positions.len() != 4 {
        warn!("mesh is not a Rectangle, can not be other shapes until 0.15");
        return;
    }

    let Some(negative_corner) =
        positions.iter().find(|arr| {
            arr[0].is_sign_negative()
                && arr[2].is_sign_negative()
        })
    else {
        warn!("unable to find negative corner");
        return;
    };
    let Some(positive_corner) =
        positions.iter().find(|arr| {
            arr[0].is_sign_positive()
                && arr[2].is_sign_positive()
        })
    else {
        warn!("unable to find positive corner");
        return;
    };

    let rect = Rectangle::from_corners(
        Vec2::new(negative_corner[0], negative_corner[2]),
        Vec2::new(positive_corner[0], positive_corner[2]),
    );

    let Ok(transform) = transforms.get(parent.get()) else {
        warn!(
            "Parent of spawn mesh didn't have a Transform"
        );
        return;
    };
    let mut rng = rand::thread_rng();
    let sample = rect.sample_interior(&mut rng);
    let mut new_transform = transform.clone();
    new_transform.translation.x += sample.x;
    new_transform.translation.z += sample.y;

    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(
            images.add(uv_debug_texture()),
        ),
        ..default()
    });

    let Ok((dropoff_entity, dropoff_transform)) =
        dropoff_locations.get_single()
    else {
        warn!("Only one dropoff location is supported");
        return;
    };
    let (character_key, random_character) =
        player_assets.character_gltfs
          .iter()
          .choose(&mut rng)
          .expect("expect random character selection to always succeed");
    let random_character_gltf =
        gltfs.get(random_character).unwrap();

    let persistent_id = PersistentId::new();
    let items = vec![
        Item {
            name: "suit".to_string(),
            owner: Some(persistent_id.clone()),
            state: ProcessedState::Unprocessed,
        },
        Item {
            name: "pen".to_string(),
            owner: Some(persistent_id.clone()),
            state: ProcessedState::Unprocessed,
        },
    ];
    commands
        .spawn((
            Name::new("CustomerNpc"),
            SpatialBundle {
                transform: new_transform,
                ..default()
            },
            // PbrBundle {
            //     mesh: meshes.add(Capsule3d::default()),
            //     material: debug_material,
            //     // transform: Transform::from_xyz(5., 2., 10.),
            //     transform: new_transform,
            //     ..default()
            // },
            CustomerNpc {
                gltf: random_character.clone(),
                expected_number_items_to_leave: items.len(),
            },
            Object(Some(dropoff_entity)),
            Path {
                current: dropoff_transform.translation,
                next: vec![],
            },
            Collider::capsule(0.5, 1.),
            Inventory {
                max_item_count: 5,
                items,
            },
            persistent_id,
        ))
        .with_children(|builder| {
            builder.spawn(SceneBundle {
                scene: random_character_gltf.scenes[0]
                    .clone(),
                transform: Transform::from_xyz(0., 0.5, 0.),
                ..default()
            });
        });
}

fn customer_spawn_cycle(
    mut commands: Commands,
    // customers: Query<&CustomerNpc>,
) {
    // if customers.iter().len() > 1 {
    //     return;
    // }
    // 1 customer per 60 * n seconds
    // because of FixedUpdate rate
    let spawn_rate = 1. / (60. * 10.);
    let mut rng = rand::thread_rng();
    // TODO: when should this become rng.random (due to gen blocks)
    if rng.r#gen::<f32>() < spawn_rate {
        commands.trigger(CustomerNpcSpawnEvent);
    }
}

fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255,
        102, 255, 121, 255, 102, 255, 102, 255, 198, 255,
        102, 198, 255, 255, 121, 102, 255, 255, 236, 102,
        255, 255,
    ];

    let mut texture_data =
        [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)]
            .copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}

fn move_customer(
    mut commands: Commands,
    mut npc_query: Query<
        (
            &mut Transform,
            &mut Path,
            Entity,
            &mut Object,
        ),
        With<CustomerNpc>,
    >,
    time: Res<Time>,
    children: Query<&Children>,
    mut transforms: Query<
        (
            &mut Transform,
            &mut AnimationTransitions,
            &mut AnimationPlayer,
        ),
        Without<CustomerNpc>,
    >,
    leaving: Query<&Leaving>,
) {
    for (mut npc_transform, mut target, entity, mut npc) in
        npc_query.iter_mut()
    {
        let move_direction =
            target.current - npc_transform.translation;
        npc_transform.translation += move_direction
            .normalize()
            * time.delta_seconds()
            * 10.0;

        // if we have a child that is an animated character
        // face them in a direction
        let Some(character_entity) = children
            .iter_descendants(entity)
            .find(|e| transforms.get(*e).is_ok())
        else {
            warn!("npc should always have a valid Transform, AnimationPlayer, and AnimationTransitions");
            continue;
        };

        let (
            mut transform,
            mut animation_transitions,
            mut player,
        ) = transforms.get_mut(character_entity).unwrap();
        let mut new_direction = -move_direction;
        new_direction.y = 0.;
        transform
            .look_to(new_direction.normalize(), Vec3::Y);

        if npc_transform
            .translation
            .distance(target.current)
            < 0.1
        {
            if let Some(next) = target.next.pop() {
                animation_transitions
                    .play(
                        &mut player,
                        CustomerNpcAnimationNames::Walk
                            .into(),
                        Duration::from_secs(0),
                    )
                    .repeat();
                target.current = next;
            } else {
                commands.entity(entity).remove::<Path>();
                let target_entity = npc.0.take().unwrap();
                // npc has made it to final target,
                // play idle animation
                // entity
                if leaving.get(entity).is_ok() {
                    commands
                        .entity(entity)
                        .despawn_recursive();
                } else {
                    animation_transitions
                        .play(
                            &mut player,
                            CustomerNpcAnimationNames::Idle
                                .into(),
                            Duration::from_secs(1),
                        )
                        .repeat();
                }
            }
        }
    }
}

fn detect_customer_dropoff(
    dropoff_sensors: Query<
        &CollidingEntities,
        With<CustomerDropoffLocation>,
    >,
    customers: Query<
        (Entity, &Inventory),
        With<CustomerNpc>,
    >,
    // TODO: a dropoff point should likely be associated
    // with some specific lights, but for now its just
    // "all of them"
    mut ready_lights: Query<
        &mut Visibility,
        With<TheLight>,
    >,
) {
    for entities_on_sensor in &dropoff_sensors {
        // if a customer is standing on the sensor and has
        // items in their inventory, then they are
        // "ready to dropoff"
        let customer =
            customers.iter().find(|(entity, inventory)| {
                entities_on_sensor.contains(entity)
                    && inventory.items.len() > 0
                    && inventory.items.iter().any(|item| item.state == ProcessedState::Unprocessed)
            });

        if customer.is_some() {
            for mut visibility in &mut ready_lights {
                *visibility = Visibility::Visible;
            }
        } else {
            for mut visibility in &mut ready_lights {
                *visibility = Visibility::Hidden;
            }
        }
    }
}

#[derive(Component)]
struct WaitingForStuffBack;

fn detect_pickup(
    query: Query<
        &CollidingEntities,
        With<CustomerDropoffLocation>,
    >,
    pickup_locations: Query<
        &CollidingEntities,
        With<PlayerReceiveFromCustomerLocation>,
    >,
    mut player: Query<
        (Entity, &mut Inventory),
        (With<Player>, Without<CustomerNpc>),
    >,
    mut customers: Query<
        (Entity, &mut Inventory),
        With<CustomerNpc>,
    >,
    mut ready_lights: Query<
        &mut Visibility,
        With<TheLight>,
    >,
    mut commands: Commands,
) {
    let Ok(pickup_colliding_entities) =
        pickup_locations.get_single()
    else {
        warn!("expected exactly 1 pickup location");
        return;
    };

    let Ok((player_entity, mut player_inventory)) =
        player.get_single_mut()
    else {
        warn!("expected exactly 1 player");
        return;
    };

    let customer_set = customers
        .iter()
        .map(|(entity, _)| entity)
        .collect::<HashSet<Entity>>();
    for sensor_colliding_entities in &query {
        let Some((customer_entity, mut customer_inventory)) =
            customers.iter_mut().find(|(entity, _)| {
                sensor_colliding_entities.contains(entity)
            })
        else {
            continue;
        };

        if pickup_colliding_entities
            .contains(&player_entity)
        {
            let available_space = player_inventory
                .max_item_count
                - player_inventory.items.len();

            // take all items, or only the amount that would
            // fit in the available space in the player's
            // inventory, whichever is smaller.
            //
            // this can be an empty range, resulting in no
            // items transferring
            let item_range = 0..(customer_inventory
                .items
                .len()
                .min(available_space));

            let transition_items =
                customer_inventory.items.drain(item_range);

            player_inventory.items.extend(transition_items);
            commands
                .entity(customer_entity)
                .insert(WaitingForStuffBack);
        }
    }
}

#[derive(Component)]
pub struct Leaving;

// TODO: customer should expect all items
fn detect_player_return_to_customer_pickup(
    dropoff_locations: Query<
        &CollidingEntities,
        With<CustomerDropoffLocation>,
    >,
    pickup_locations: Query<
        &CollidingEntities,
        With<PlayerReceiveFromCustomerLocation>,
    >,
    mut player: Query<
        (Entity, &mut Inventory),
        (With<Player>, Without<CustomerNpc>),
    >,
    mut customers: Query<
        (
            Entity,
            &mut Inventory,
            &PersistentId,
            &CustomerNpc,
        ),
        (With<WaitingForStuffBack>,),
    >,
    mut ready_lights: Query<
        &mut Visibility,
        With<TheLight>,
    >,
    spawner_meshes: Query<
        (Entity, &Parent),
        With<CustomerNpcSpawner>,
    >,
    transforms: Query<&Transform>,
    mut commands: Commands,
) {
    let Ok(pickup_colliding_entities) =
        pickup_locations.get_single()
    else {
        warn!("expected exactly 1 pickup location");
        return;
    };

    let Ok((player_entity, mut player_inventory)) =
        player.get_single_mut()
    else {
        warn!("expected exactly 1 player");
        return;
    };

    for sensor_colliding_entities in &dropoff_locations {
        for (
            customer_entity,
            mut customer_inventory,
            customer_persistent_id,
            customer_npc,
        ) in customers.iter_mut().filter(
            |(entity, _, _, _)| {
                sensor_colliding_entities.contains(entity)
            },
        ) {
            if pickup_colliding_entities
                .contains(&player_entity)
            {
                // move each item from player_inventory to
                // customer_inventory
                //
                // another option is to use extract_if, but
                // its nightly
                //
                // the index here needs to be accounted for
                // separately because .remove() will change
                // the indices and shift all future elements
                // to the left
                // let mut i = 0;
                // while i < player_inventory.items.len() {
                //     if customer_inventory
                //         .has_available_space()
                //         && player_inventory.items[i]
                //             .owner
                //             .as_ref()
                //             == Some(customer_persistent_id)
                //         && player_inventory.items[i].state
                //             == ProcessedState::Processed
                //     {
                //         let val = player_inventory
                //             .items
                //             .remove(i);
                //         customer_inventory.items.push(val);
                //     } else {
                //         i += 1;
                //     }
                // }

                let available_space = customer_inventory
                    .max_item_count
                    - customer_inventory.items.len();
                for item_to_move in player_inventory
                    .items
                    .extract_if(|item| {
                        item.owner.as_ref()
                            == Some(customer_persistent_id)
                            && item.state
                                == ProcessedState::Processed
                    })
                    .take(available_space)
                {
                    customer_inventory
                        .items
                        .push(item_to_move);
                }

                if customer_inventory.items.len()
                    == customer_npc
                        .expected_number_items_to_leave
                    && customer_inventory.items.iter().all(
                        |item| {
                            item.state
                                == ProcessedState::Processed
                        },
                    )
                {
                    let Some((exit_entity, exit_parent)) =
                        spawner_meshes.iter().next()
                    else {
                        warn!("no way to leave");
                        return;
                    };
                    let Ok(exit_transform) =
                        transforms.get(exit_parent.get())
                    else {
                        warn!("no exit for customer");
                        return;
                    };
                    info!(
                        ?exit_entity,
                        location = ?exit_transform.translation,
                        "trying to exit"
                    );
                    commands
                        .entity(customer_entity)
                        .insert(Object(Some(exit_entity)))
                        .insert(Path {
                            current: exit_transform
                                .translation,
                            next: vec![],
                        })
                        .insert(Leaving);
                }
            }
        }
    }
}
