use bevy::{prelude::*, utils::HashMap};

pub struct AnimationGraphProcessingPlugin;

impl Plugin for AnimationGraphProcessingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LoadedAnimationGraphs(
            HashMap::default(),
        ))
        .add_systems(Update, process_loaded_gltf);
    }
}
#[derive(Component, Clone)]
pub struct AnimationsList(pub Vec<AnimationNodeIndex>);

#[derive(Resource, Deref)]
pub struct LoadedAnimationGraphs(
    pub  HashMap<
        String,
        (AnimationsList, Handle<AnimationGraph>),
    >,
);

fn process_loaded_gltf(
    mut events: EventReader<AssetEvent<Gltf>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    names: Query<&Name>,
    gltfs: Res<Assets<Gltf>>,
    mut loaded_animation_graphs: ResMut<
        LoadedAnimationGraphs,
    >,
) {
    for event in events.read() {
        let AssetEvent::LoadedWithDependencies { id } =
            event
        else {
            continue;
        };

        let Some(gltf) = gltfs.get(*id) else {
            error!("gltf is not loaded, even though we're in AssetEvent::LoadedWithDependencies");
            continue;
        };

        let Some(name) = gltf
            .named_nodes
            .keys()
            .find(|node| node.starts_with("character-"))
        else {
            continue;
        };
        // info!(?name);
        // character-female-a

        let mut graph = AnimationGraph::new();
        let animations = AnimationsList(
            graph
                .add_clips(
                    gltf.animations.clone(),
                    1.0,
                    graph.root,
                )
                .collect(),
        );
        let graph = graphs.add(graph);
        loaded_animation_graphs
            .0
            .insert(name.to_string(), (animations, graph));
    }
}
