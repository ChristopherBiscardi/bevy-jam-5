use bevy::{
    ecs::{
        component::{ComponentHooks, StorageType},
        world::DeferredWorld,
    },
    pbr::MaterialExtension,
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    scene::SceneInstanceReady,
};

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct TileFloorMaterial {
    // We need to ensure that the bindings of the base
    // material and the extension do not conflict,
    // so we start from binding slot 100, leaving slots
    // 0-99 for the base material.
    #[uniform(100)]
    pub quantize_steps: u32,
}

impl MaterialExtension for TileFloorMaterial {
    fn fragment_shader() -> ShaderRef {
        "custom_materials/tile_floor.wgsl".into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        "custom_materials/tile_floor.wgsl".into()
    }
}
