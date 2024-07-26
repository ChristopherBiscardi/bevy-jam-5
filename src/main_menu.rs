use std::time::{Duration, Instant};

use bevy::{
    color::palettes::tailwind::{
        SLATE_200, SLATE_300, SLATE_400, SLATE_50,
        SLATE_500, SLATE_600,
    },
    prelude::*,
    render::view::RenderLayers,
};
use bevy_mod_picking::{
    prelude::*,
    // picking_core::Pickable
};
use woodpecker_ui::prelude::*;

use crate::{
    assets::{FontAssets, FontVelloAssets},
    widgets::*,
    AppState,
};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::MainMenu),
            spawn_main_menu,
        );
    }
}

fn spawn_main_menu(
    mut commands: Commands,
    mut ui_context: ResMut<WoodpeckerContext>,
    fonts: Res<FontVelloAssets>,
    mut font_manager: ResMut<FontManager>,
    asset_server: Res<AssetServer>,
) {
    let mut buttons = WidgetChildren::default();
    buttons.add::<MainMenuButtonWidget>((
        MainMenuButtonWidgetBundle {
            props: MainMenuButtonWidget {
                content: "New Game".to_string(),
                ..default()
            },
            ..default()
        },
        On::<Pointer<Click>>::run(
            |mut next_state: ResMut<NextState<AppState>>| {
                info!("clicked");
                next_state.set(AppState::InGame);
            },
        ),
    ));
    buttons.add::<MainMenuButtonWidget>((
        MainMenuButtonWidgetBundle {
            props: MainMenuButtonWidget {
                content: "Options".to_string(),
                ..default()
            },
            ..default()
        },
        On::<Pointer<Click>>::run(
            |mut commands: Commands| {
                info!("clicked");
                // commands.trigger()
            },
        ),
    ));
    buttons.add::<MainMenuButtonWidget>((
        MainMenuButtonWidgetBundle {
            props: MainMenuButtonWidget {
                content: "Exit".to_string(),
                ..default()
            },
            ..default()
        },
        On::<Pointer<Click>>::run(
            |mut commands: Commands, mut exit: EventWriter<AppExit>| {
                exit.send(AppExit::Success);
            },
        ),
   ) );



    let root = commands
        .spawn((
            StateScoped(AppState::MainMenu),
            WoodpeckerAppBundle {
                children: WidgetChildren::default().with_child::<Element>(ElementBundle {
                    styles: WoodpeckerStyle {
                        width: Units::Percentage(100.0),
                        height: Units::Percentage(100.0),
                        justify_content: Some(WidgetAlignContent::FlexStart),
                        align_content: Some(WidgetAlignContent::Center),
                        padding: Edge {
                            left: 0.0.into(),
                            right: 0.0.into(),
                            top: 25.0.into(),
                            bottom: 0.0.into(),
                        },
                        ..Default::default()
                    },
   
                    children: WidgetChildren::default().with_child::<Element>((
                        ElementBundle {
                            styles: WoodpeckerStyle {
                                background_color: Srgba::hex("FF007F").unwrap().into(),
                                margin: Edge { top: Units::Pixels(25.), right: Units::Pixels(25.), bottom: Units::Pixels(25.), left: Units::Pixels(25.) },
                                width: Units::Pixels(300.),
                                height: Units::Pixels(300.),
                                gap: (Units::Pixels(10.), Units::Pixels(5.)),
                                justify_content: Some(WidgetAlignContent::Center),
                                align_content: Some(WidgetAlignContent::Center),
                                display: WidgetDisplay::Flex,
                                flex_direction: WidgetFlexDirection::Column,
                                position: WidgetPosition::Relative,
                                // flex_wrap: WidgetFlexWrap::Wrap,
                                ..Default::default()
                            },
                            children: buttons,
                            ..Default::default()
                        },
                    )),
                    ..Default::default()
                }),
                ..Default::default()
            },
        ))
        .id();

    let mut root_children = WidgetChildren::default();

    ui_context.set_root_widget(root);
}
