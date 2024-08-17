use std::{
    ops::Deref,
    time::{Duration, Instant},
};

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
use vello::wgpu::{
    Extent3d, TextureDimension, TextureFormat,
};
use woodpecker_ui::prelude::*;

use crate::{
    assets::{FontAssets, FontVelloAssets},
    game_scene::Player,
    navmesh::{Object, Path, SpawnObstacle},
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
            .register_type::<Inventory>()
            .add_systems(
                Update,
                (
                    move_customer,
                    detect_customer_dropoff,
                    detect_pickup
                )
                    .run_if(in_state(IsPaused::Running)),
            )
            .observe(spawn_customer_npc);
    }
}

#[derive(Component)]
pub struct CustomerNpc;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Inventory {
    pub max_item_count: usize,
    pub items: Vec<String>,
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
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Capsule3d::default()),
            material: debug_material,
            // transform: Transform::from_xyz(5., 2., 10.),
            transform: new_transform,
            ..default()
        },
        CustomerNpc,
        Object(Some(dropoff_entity)),
        Path {
            current: dropoff_transform.translation,
            next: vec![],
        },
        Collider::capsule(0.5, 1.),
        Inventory {
            max_item_count: 5,
            items: vec![
                "suit".to_string(),
                "pen".to_string(),
            ],
        },
    ));
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
    mut object_query: Query<
        (
            &mut Transform,
            &mut Path,
            Entity,
            &mut Object,
        ),
        With<CustomerNpc>,
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
    // TODO: a dropoff point should likely be associated with
    // some specific lights, but for now its just "all of them"
    mut ready_lights: Query<
        &mut Visibility,
        With<TheLight>,
    >,
) {
    for entities_on_sensor in &dropoff_sensors {
        // if a customer is standing on the sensor and has items
        // in their inventory, then they are "ready to dropoff"
        let customer =
            customers.iter().find(|(entity, inventory)| {
                entities_on_sensor.contains(entity)
                    && inventory.items.len() > 0
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
        }
    }
}
// fn print_started_collisions() {
//     for CollisionStarted(entity1, entity2) in
//         collision_event_reader.read()
//     {
//         println!(
//             "Entities {:?} and {:?} started colliding",
//             entity1, entity2,
//         );
//     }
// }
