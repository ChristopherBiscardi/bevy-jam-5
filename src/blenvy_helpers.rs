use bevy::{
    ecs::{
        component::{
            ComponentHooks, ComponentId, StorageType,
        },
        world::DeferredWorld,
    },
    prelude::*,
};

pub struct BlenvyHelpersPlugin;

impl Plugin for BlenvyHelpersPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<NameFromBlender>();
    }
}

#[derive(Reflect, Debug, Clone)]
#[reflect(Debug, Component)]
pub struct NameFromBlender {
    name: String,
}

impl Component for NameFromBlender {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    fn register_component_hooks(
        hooks: &mut ComponentHooks,
    ) {
        hooks.on_add(
            |mut world: DeferredWorld,
             entity: Entity,
             _id: ComponentId| {
                let NameFromBlender { name } = world
                    .get::<NameFromBlender>(entity)
                    .unwrap()
                    .clone();
                world
                    .commands()
                    .entity(entity)
                    .insert(Name::new(name))
                    .remove::<NameFromBlender>();
            },
        );
    }
}
