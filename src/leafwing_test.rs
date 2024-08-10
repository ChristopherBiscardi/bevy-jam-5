use bevy::{
    ecs::component::{ComponentHooks, StorageType},
    prelude::*,
};
use leafwing_input_manager::prelude::*;

pub struct LeafwingTestPlugin;

impl Plugin for LeafwingTestPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            InputManagerPlugin::<PlayerAction>::default(),
        )
        .add_systems(Update, player_walks)
        .register_type::<BoxPlayer>();
    }
}

#[derive(
    Actionlike,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Hash,
    Debug,
    Reflect,
)]
enum PlayerAction {
    // Movement
    Up,
    Down,
    Left,
    Right,
    // Abilities
    Ability1,
    Ability2,
    Ability3,
    Ability4,
    Ultimate,
}

impl PlayerAction {
    // Lists like this can be very useful for quickly
    // matching subsets of actions
    const DIRECTIONS: [Self; 4] = [
        PlayerAction::Up,
        PlayerAction::Down,
        PlayerAction::Left,
        PlayerAction::Right,
    ];

    fn direction(self) -> Option<Dir2> {
        match self {
            PlayerAction::Up => Some(Dir2::Y),
            PlayerAction::Down => Some(Dir2::NEG_Y),
            PlayerAction::Left => Some(Dir2::NEG_X),
            PlayerAction::Right => Some(Dir2::X),
            _ => None,
        }
    }
    fn default_input_map() -> InputMap<PlayerAction> {
        // This allows us to replace `PlayerAction::Up`
        // with `Up`, significantly reducing
        // boilerplate
        use PlayerAction::*;
        let mut input_map = InputMap::default();

        // Movement
        input_map.insert(Up, KeyCode::ArrowUp);
        input_map.insert(Up, GamepadButtonType::DPadUp);

        input_map.insert(Down, KeyCode::ArrowDown);
        input_map.insert(Down, GamepadButtonType::DPadDown);

        input_map.insert(Left, KeyCode::ArrowLeft);
        input_map.insert(Left, GamepadButtonType::DPadLeft);

        input_map.insert(Right, KeyCode::ArrowRight);
        input_map
            .insert(Right, GamepadButtonType::DPadRight);

        // Abilities
        input_map.insert(Ability1, KeyCode::KeyQ);
        input_map.insert(Ability1, GamepadButtonType::West);
        input_map.insert(Ability1, MouseButton::Left);

        input_map.insert(Ability2, KeyCode::KeyW);
        input_map
            .insert(Ability2, GamepadButtonType::North);
        input_map.insert(Ability2, MouseButton::Right);

        input_map.insert(Ability3, KeyCode::KeyE);
        input_map.insert(Ability3, GamepadButtonType::East);

        input_map.insert(Ability4, KeyCode::Space);
        input_map
            .insert(Ability4, GamepadButtonType::South);

        input_map.insert(Ultimate, KeyCode::KeyR);
        input_map.insert(
            Ultimate,
            GamepadButtonType::LeftTrigger2,
        );

        input_map
    }
}

#[derive(Reflect)]
#[reflect(Component)]
struct BoxPlayer;
impl Component for BoxPlayer {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(
        hooks: &mut ComponentHooks,
    ) {
        hooks.on_add(|mut world, entity, _| {
            world
                .commands()
                .entity(entity)
                .insert(InputManagerBundle::with_map(
                    PlayerAction::default_input_map(),
                ))
                .remove::<BoxPlayer>();
        });
    }
}

fn player_walks(
    query: Query<
        &ActionState<PlayerAction>,
        // With<BoxPlayer>,
    >,
) {
    for action_state in &query {
        let mut direction_vector = Vec2::ZERO;

        for input_direction in PlayerAction::DIRECTIONS {
            if action_state.pressed(&input_direction) {
                if let Some(direction) =
                    input_direction.direction()
                {
                    // Sum the directions as 2D vectors
                    direction_vector += *direction;
                }
            }
        }

        // Then reconvert at the end, normalizing the
        // magnitude
        let net_direction = Dir2::new(direction_vector);

        if let Ok(direction) = net_direction {
            info!(?direction, "box goes");
        }
    }
}
