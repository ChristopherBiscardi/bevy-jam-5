use bevy::{
    color::palettes::tailwind::PINK_400,
    ecs::component::{ComponentHooks, StorageType},
    prelude::*,
};
use bevy_tnua::{
    prelude::{
        TnuaBuiltinJump, TnuaBuiltinWalk, TnuaController,
    },
    TnuaUserControlsSystemSet,
};
use leafwing_input_manager::prelude::*;

use crate::{
    camera::GameCamera,
    game_scene::Player,
    states::IsPaused,
    widgets::{InventoryModal, OptionsModal},
};

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            InputManagerPlugin::<PlayerAction>::default(),
        )
        .add_systems(
            Update,
            (
                handle_inventory
                    .run_if(in_state(IsPaused::Running)),
                apply_controls
                    .in_set(TnuaUserControlsSystemSet)
                    .run_if(in_state(IsPaused::Running)),
            ),
        );
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
pub enum PlayerAction {
    // Movement
    Up,
    Down,
    Left,
    Right,
    //
    Pause,
    Inventory,
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

    pub fn default_input_map() -> InputMap<PlayerAction> {
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

        //

        input_map.insert(Inventory, KeyCode::KeyI);

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

fn handle_inventory(
    mut query: Query<
        &ActionState<PlayerAction>,
        With<Player>,
    >,
    mut modal: Query<&mut InventoryModal>,
) {
    for player_action in &query {
        if player_action
            .just_pressed(&PlayerAction::Inventory)
        {
            let Ok(mut modal) = modal.get_single_mut()
            else {
                warn!("Expected a single modal");
                return;
            };
            modal.show_modal = !modal.show_modal;
        }
    }
}

fn apply_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<
        (
            &mut TnuaController,
            &ActionState<PlayerAction>,
        ),
        With<Player>,
    >,
    main_camera: Query<&Transform, With<GameCamera>>,
    mut gizmos: Gizmos,
) {
    let Ok(camera) = main_camera.get_single() else {
        warn!("wrong number of cameras");
        return;
    };
    for (mut controller, action_state) in &mut query {
        let mut direction_vector = Vec2::ZERO;

        for input_direction in PlayerAction::DIRECTIONS {
            if action_state.pressed(&input_direction) {
                let direction = match input_direction {
                    PlayerAction::Up => {
                        camera.forward().xz()
                    }
                    PlayerAction::Down => {
                        -camera.forward().xz()
                    }
                    PlayerAction::Left => {
                        -camera.right().xz()
                    }
                    PlayerAction::Right => {
                        camera.right().xz()
                    }
                    _ => {
                        continue;
                    }
                };

                // Sum the directions as 2D vectors
                direction_vector += direction.normalize();
            }
        }

        let dir = direction_vector.normalize();

        let desired_velocity = Vec3::new(
            if dir.x.is_nan() { 0. } else { dir.x },
            0.,
            if dir.y.is_nan() { 0. } else { dir.y },
        ) * 10.;
        // Feed the basis every frame. Even if the player
        // doesn't move - just use `desired_velocity:
        // Vec3::ZERO`. `TnuaController` starts without a
        // basis, which will make the character collider
        // just fall.
        controller.basis(TnuaBuiltinWalk {
            // The `desired_velocity` determines how the
            // character will move.
            desired_velocity,
            // The `float_height` must be greater (even if
            // by little) from the distance
            // between the character's center
            // and the lowest point of its
            // collider.
            float_height: 1.5,
            // `TnuaBuiltinWalk` has many other fields for
            // customizing the movement - but they have
            // sensible defaults. Refer to the
            // `TnuaBuiltinWalk`'s documentation to learn
            // what they do.
            desired_forward: -desired_velocity.normalize(),
            ..default()
        });

        // Feed the jump action every frame as long as the
        // player holds the jump button. If the player
        // stops holding the jump button, simply stop
        // feeding the action.
        if action_state.pressed(&PlayerAction::Ability4) {
            controller.action(TnuaBuiltinJump {
                // The height is the only mandatory field of
                // the jump button.
                height: 4.0,
                // `TnuaBuiltinJump` also has customization
                // fields with sensible defaults.
                ..default()
            });
        }
    }
}
