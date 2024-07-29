use avian3d::prelude::*;
use bevy::{
    app::Plugin,
    ecs::{
        component::{
            ComponentHooks, ComponentId, StorageType,
        },
        world::DeferredWorld,
    },
    prelude::{Component, Entity, ReflectComponent},
    reflect::Reflect,
};

pub struct CollisionLayersPlugin;

impl Plugin for CollisionLayersPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<CollisionGrouping>();
    }
}

#[derive(PhysicsLayer)]
pub enum GameLayer {
    Player, // Layer 0
    Enemy,  // Layer 1
    Ground, // Layer 2
}

#[derive(Reflect, Debug)]
#[reflect(Debug, Component)]
pub enum CollisionGrouping {
    Environment,
    Player,
    Enemy,
}

impl Component for CollisionGrouping {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    fn register_component_hooks(
        hooks: &mut ComponentHooks,
    ) {
        hooks.on_add(
            |mut world: DeferredWorld,
             entity: Entity,
             _id: ComponentId| {
                let value = world
                    .get::<CollisionGrouping>(entity)
                    .unwrap();
                match value {
                    CollisionGrouping::Environment => {
                        world
                            .commands()
                            .entity(entity)
                            .insert(CollisionLayers::new(
                                GameLayer::Ground,
                                [
                                    GameLayer::Enemy,
                                    GameLayer::Player,
                                ],
                            ))
                            .remove::<CollisionGrouping>();
                    }
                    CollisionGrouping::Player => {
                        world
                            .commands()
                            .entity(entity)
                            .insert(CollisionLayers::new(
                                GameLayer::Player,
                                [
                                    GameLayer::Enemy,
                                    GameLayer::Ground,
                                ],
                            ))
                            .remove::<CollisionGrouping>();
                    }
                    CollisionGrouping::Enemy => {
                        world
                            .commands()
                            .entity(entity)
                            .insert(CollisionLayers::new(
                                GameLayer::Enemy,
                                [
                                    GameLayer::Player,
                                    GameLayer::Ground,
                                ],
                            ))
                            .remove::<CollisionGrouping>();
                    }
                }
            },
        );
    }
}
