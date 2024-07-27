use bevy::prelude::*;
use main_menu_button::main_menu_interaction;
use woodpecker_ui::prelude::*;

mod main_menu_button;
pub use main_menu_button::{
    MainMenuButtonWidget, MainMenuButtonWidgetBundle,
};
pub mod timer_transition;
pub use timer_transition::TransitionTimer;
pub mod modal;
pub use modal::{OptionsModal, OptionsModalBundle};

pub struct CustomWidgetsPlugin;

impl Plugin for CustomWidgetsPlugin {
    fn build(&self, app: &mut App) {
        app.register_widget::<MainMenuButtonWidget>()
            .register_widget::<OptionsModal>()
            // .register_widget::<TransitionTimer>()
            .add_systems(
                Update,
                (
                    timer_transition::update_transitions,
                    main_menu_interaction,
                ),
            );
    }
}
