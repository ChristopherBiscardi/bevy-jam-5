use std::time::{Duration, Instant};

use bevy::{
    color::palettes::tailwind::*, prelude::*,
    render::view::RenderLayers,
};
use bevy_mod_picking::prelude::*;
use woodpecker_ui::prelude::*;

use crate::{
    assets::{FontAssets, FontVelloAssets}, spawn_2d_camera, widgets::{self, *}, AppState
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
                offset: 200,
                ..default()
            },
            ..default()
        },
        On::<Pointer<Click>>::run(
            |mut next_state: ResMut<
                NextState<AppState>,
            >| {
                next_state.set(AppState::InGame);
            },
        ),
    ));
    buttons.add::<MainMenuButtonWidget>((
        MainMenuButtonWidgetBundle {
            props: MainMenuButtonWidget {
                content: "Options".to_string(),
                offset: 250,
                ..default()
            },
            ..default()
        },
        On::<Pointer<Click>>::run(
            |mut commands: Commands, mut modal: Query<&mut OptionsModal>| {
                let Ok(mut modal) = modal.get_single_mut() else {
                    warn!("Expected a single modal");
                    return;
                };
                modal.show_modal = true;
            },
        ),
    ));
    buttons.add::<MainMenuButtonWidget>((
        MainMenuButtonWidgetBundle {
            props: MainMenuButtonWidget {
                content: "Exit".to_string(),
                offset: 300,
                ..default()
            },
            ..default()
        },
        On::<Pointer<Click>>::run(
            |mut commands: Commands,
             mut exit: EventWriter<AppExit>| {
                exit.send(AppExit::Success);
            },
        ),
    ));

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
                        ..default()
                    },
   
                    children: WidgetChildren::default().with_child::<Element>(
                       (
                            Name::new("MainMenu::Title"),
                            ElementBundle::default(),
                            WidgetRender::Text {
                                content: "Wash Cycle".to_string(),
                                word_wrap: false,
                            },
                            TransitionTimer {
                                easing: widgets::timer_transition::TransitionEasing::QuinticOut,
                                start: Timer::new(
                                    Duration::from_millis(0),
                                    TimerMode::Once,
                                ),
                                timeouts: vec![
                                    Timer::new(
                                        Duration::from_millis(200),
                                        TimerMode::Once,
                                    ),
                                    Timer::new(
                                        Duration::from_millis(200),
                                        TimerMode::Once,
                                    ),
                                    Timer::new(
                                        Duration::from_millis(200),
                                        TimerMode::Once,
                                    )
                                ],
                                looping: false,
                                styles: vec![WoodpeckerStyle {
                                    position: WidgetPosition::Absolute,
                                    margin: Edge::all(50.),
                                    font_size: 125.0,
                                    color: SLATE_950.with_alpha(0.).into(),
                                    font: Some(fonts.outfit_bold.id()),
                                    left: Units::Percentage(-25.),
                                    top: Units::Percentage(45.),
                                    ..default()
                                },
                                WoodpeckerStyle {
                                    position: WidgetPosition::Absolute,
                                    margin: Edge::all(50.),
                                    font_size: 125.0,
                                    color: SLATE_950.into(),
                                    font: Some(fonts.outfit_bold.id()),
                                    left: Units::Percentage(50.),
                                    top: Units::Percentage(45.),
                                    ..default()
                                },
                                WoodpeckerStyle {
                                    position: WidgetPosition::Absolute,
                                    margin: Edge::all(50.),
                                    font_size: 125.0,
                                    color: SLATE_950.into(),
                                    font: Some(fonts.outfit_bold.id()),
                                    left: Units::Percentage(50.),
                                    top: Units::Percentage(45.),
                                    ..default()
                                },
                                WoodpeckerStyle {
                                    position: WidgetPosition::Absolute,
                                    margin: Edge::all(50.),
                                    font_size: 125.0,
                                    color: SLATE_950.into(),
                                    font: Some(fonts.outfit_bold.id()),
                                    left: Units::Percentage(0.),
                                    top: Units::Percentage(0.),
                                    ..default()
                                }],
                                ..default()
                            }
                        )
                    ).with_child::<Element>((
                        ElementBundle {
                            styles: WoodpeckerStyle {
                                background_color: Srgba::hex("FF007F").unwrap().into(),
                                margin: Edge { top: Units::Percentage(25.), right: Units::Pixels(25.), bottom: Units::Pixels(25.), left: Units::Percentage(15.) },
                                width: Units::Pixels(300.),
                                height: Units::Pixels(300.),
                                gap: (Units::Pixels(10.), Units::Pixels(5.)),
                                justify_content: Some(WidgetAlignContent::Center),
                                align_content: Some(WidgetAlignContent::Center),
                                display: WidgetDisplay::Flex,
                                flex_direction: WidgetFlexDirection::Column,
                                position: WidgetPosition::Relative,
                                // flex_wrap: WidgetFlexWrap::Wrap,
                                ..default()
                            },
                            children: buttons,
                            ..default()
                        },
                    ))
                    .with_child::<OptionsModal>(
                        OptionsModalBundle {
                            styles: WoodpeckerStyle {
                                width: Units::Percentage(100.0),
                                justify_content: Some(
                                    WidgetAlignContent::Center,
                                ),
                                ..default()
                            },
                            ..default()
                        },
                    ),
                    ..default()
                }),
                ..default()
            },
        ))
        .id();

    let mut root_children = WidgetChildren::default();

    ui_context.set_root_widget(root);
}
