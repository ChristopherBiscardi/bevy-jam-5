use bevy::{
    ecs::{
        component::{ComponentHooks, StorageType},
        world::DeferredWorld,
    },
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    scene::SceneInstanceReady,
};

pub struct CustomMaterialsPlugin;

impl Plugin for CustomMaterialsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            MaterialPlugin::<ItemPickupMaterial>::default(),
        )
        .register_type::<CustomMaterialReplacement>()
        .add_systems(
            Update,
            replace_blenvy_materials
                .run_if(on_event::<SceneInstanceReady>()),
        );
    }
}

fn replace_blenvy_materials(
    mut commands: Commands,
    mut events: EventReader<SceneInstanceReady>,
    materials_to_replace: Query<
        (Entity, &CustomMaterialReplacement),
        With<Handle<StandardMaterial>>,
    >,
    mut materials_item_pickup: ResMut<
        Assets<ItemPickupMaterial>,
    >,
) {
    for event in events.read() {
        for (entity, material_to_use) in
            &materials_to_replace
        {
            match material_to_use {
                CustomMaterialReplacement::ItemPickup => {
                    commands
                        .entity(entity)
                        .remove::<Handle<StandardMaterial>>(
                        )
                        .insert(materials_item_pickup.add(
                            ItemPickupMaterial {
                                color: LinearRgba::BLUE,
                                color_texture: None,
                                alpha_mode:
                                    AlphaMode::Blend,
                            },
                        ));
                }
            }
        }
    }
}

#[derive(Reflect, Component, Debug, Clone)]
#[reflect(Debug, Component)]
pub enum CustomMaterialReplacement {
    ItemPickup,
}

// This struct defines the data that will be
// passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct ItemPickupMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
    alpha_mode: AlphaMode,
}

/// The Material trait is very configurable, but
/// comes with sensible defaults for all methods.
/// You only need to implement functions for
/// features that need non-default behavior. See
/// the Material api docs for details!
impl Material for ItemPickupMaterial {
    fn fragment_shader() -> ShaderRef {
        "custom_materials/item_pickup.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }

    fn specialize(
            pipeline: &bevy::pbr::MaterialPipeline<Self>,
            descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
            layout: &bevy::render::mesh::MeshVertexBufferLayoutRef,
            key: bevy::pbr::MaterialPipelineKey<Self>,
    ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError>{
        descriptor.primitive.cull_mode = None;
        Ok(())
    }
}